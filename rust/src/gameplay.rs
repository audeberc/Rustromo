use godot::prelude::*;
use std::collections::{HashMap, VecDeque};
use crate::objectives::{Objectives, Objective};
use crate::map::{GameMap, Item}; // Import Item

use crate::player::Player;
use rand;

#[derive(GodotClass)]
#[class(base = Node)]
struct Gameplay {
    base: Base<Node>,
    objectives: Vec<Objective>,
}

#[godot_api]
impl INode for Gameplay {
    fn init(base: Base<Node>) -> Self {
        godot_print!("Gameplay initialized");
        let objectives = Objectives::new().objectives;
        Self { base, objectives }
    }
}

#[godot_api]
impl Gameplay {


    // Gets the possible movements for the current turn
    #[func]
    fn get_possible_movements(&self, map: Gd<GameMap>, player: Gd<Player>, alien: Gd<Player>) -> PackedStringArray {
        let map = map.bind();
        let player = player.bind();
        let alien = alien.bind();
        let connected_rooms = map.get_connected_rooms(player.get_current_room_index());
        let mut movements = PackedStringArray::new();

        if player.get_current_room_index() == alien.get_current_room_index() {
            // Restrict movements to rooms 3 tiles away
            let distant_rooms = map.get_rooms_within_distance(player.get_current_room_index() as usize, 3, 3);
            for room_index in distant_rooms.iter() {
                let instruction = format!("flee_to {}", room_index);
                movements.push(&instruction);
            }
            
        } else {
            if player.get_remaining_actions() > 0 {
                for room_index in connected_rooms.iter() {
                    let instruction = format!("move_to {}", room_index);
                    movements.push(&instruction);
                }
                let items = player.get_item_slots();
                godot_print!("Items: {:?}", items);
                for item_dict in items.iter() {
                    let item_name = item_dict.get("name").expect("Item name not found").to_string();
                    let alien_room = alien.get_current_room_index();
                    let player_room = player.get_current_room_index();
                    let distant_rooms = map.get_rooms_within_distance(player_room as usize, 1, 3);

                    if item_name == "flamethrower" {
                        if distant_rooms.contains(&(alien_room as usize)) {
                            let instruction = format!("use_item {}", item_name);
                            movements.push(&instruction);
                        }
                    } else if item_name == "flare" {
                        if distant_rooms.contains(&(alien_room as usize)) {
                            let alien_distant_rooms = map.get_rooms_within_distance(alien.get_current_room_index() as usize, 3, 3);
           
                            for room_index in alien_distant_rooms.iter() {
                                // It would be stupid to drop the flare at your feet ? 
                                if room_index != &(player_room as usize) {
                                let instruction = format!("use_item {} {}", item_name, room_index);
                                movements.push(&instruction);}
                            }
                        }
                    } else {
                        let room_limitation_name = item_dict.get("room_limitation_name").expect("Item name not found").to::<String>();
                        godot_print!("Room limitation name: {}", room_limitation_name);
                        if room_limitation_name == ""|| room_limitation_name == map.get_room_name(player_room) {
                            let instruction = format!("use_item {}", item_name);
                            movements.push(&instruction);
                        }
                    }
                }
                // Add option to pick up scrap tokens
                let current_room_index = player.get_current_room_index();
                let scrap_tokens = map.get_scrap_tokens_in_room(current_room_index);
                if scrap_tokens > 0 {
                    let instruction = format!("pick_up_scrap {}", scrap_tokens);
                    movements.push(&instruction);
                }
                // Add option to pick up items in the room
                let room_objects = map.get_room_objects(current_room_index);
                for object in room_objects.iter() {
                    let instruction = format!("pick_up_item {}", object.name);
                    movements.push(&instruction);
                }
            }
            movements.push("end_turn");
        }

        movements
    }
    // Creates the text string for the current room info
    #[func]
    fn create_room_info_text(&self, map: Gd<GameMap>, player: Gd<Player>) -> String {
        let map = map.bind();
        let player = player.bind();
        let mut info = String::new();
        info.push_str(&format!("{}\n", map.get_room_info(player.get_current_room_index())));
        info
    }

    // Creates the text string for the player's info
    #[func]
    fn create_player_info_text(&self, player: Gd<Player>) -> String {
        let player = player.bind();
        let mut info = String::new();
        info.push_str(&format!("Remaining actions: {}\n", player.get_remaining_actions()));
        info.push_str(&format!("Morale: {} %\n", player.get_morale()));
        info.push_str(&format!("Scrap tokens: {}\n", player.get_scraps()));

        let items = player.get_item_slots();
        info.push_str("Inventory:\n");
        for item_dict in items.iter() {
            let item_name = item_dict.get("name").expect("Item name not found").to::<String>();
            let item_uses = item_dict.get("uses").expect("Item uses not found").to::<i32>();
            info.push_str(&format!("{}: {} uses\n", item_name, item_uses));
        }

        info
    }

    // Handles the selected item based on the instruction provided by Godot
    #[func]
    fn handle_selected_item(&mut self, mut map: Gd<GameMap>, mut player: Gd<Player>, mut alien: Gd<Player>, instruction: String) -> String {
        let mut map = map.bind_mut();
        let mut player = player.bind_mut();
        let mut alien = alien.bind_mut();

    
        if instruction == "end_turn" || player.get_remaining_actions() == 0 {
            // Handle end turn logic
            godot_print!("End Turn selected");

            player.end_turn(); // Reset actions for the next turn
            alien.end_turn();
            // Pick a random number of steps for the alien to take
            let movement_range_categories = [1, 1, 1, 2, 2, 3];
            let steps = movement_range_categories[rand::random::<usize>() % movement_range_categories.len()];
            self.move_alien(&map, &mut player, &mut alien, steps);

            // Place scrap tokens in the current room
            let scrap_amount_category = [1, 1, 1, 2, 2, 3];
            let scrap_amount =  scrap_amount_category[rand::random::<usize>() % movement_range_categories.len()];
            
            map.add_scrap_to_room(player.get_current_room_index(), scrap_amount);
            godot_print!("Placed {} scrap tokens in room {}", scrap_amount, player.get_current_room_index());
        }

        else if instruction.starts_with("move_to") {
            let parts: Vec<&str> = instruction.split_whitespace().collect();
            if parts.len() == 2 {
                if let Ok(room_index) = parts[1].parse::<usize>() {
                    // Handle move to room logic
                    godot_print!("Move to room {}", room_index);
                    player.move_to_room(room_index.try_into().unwrap());
                }
            }
        }
        else if instruction.starts_with("flee_to") {
            let parts: Vec<&str> = instruction.split_whitespace().collect();
            if parts.len() == 2 {
                if let Ok(room_index) = parts[1].parse::<usize>() {
                    godot_print!("Flee to room {}", room_index);
                    player.end_turn(); // recharge the actions to make sure fleeing is possible
                    player.move_to_room(room_index.try_into().unwrap());
                    // Pick morale damage from predefined categories
                    let morale_damage_categories = [10.0, 10.0, 10.0, 15.0, 20.0, 25.0];
                    let morale_damage = morale_damage_categories[rand::random::<usize>() % morale_damage_categories.len()];
                    let morale = player.get_morale() - morale_damage;
                    player.set_morale(morale);
                    if morale <= 0.0 {
                        godot_print!("GAME OVER: Player's morale reached 0");
                        return "game_over".to_string();
                    } else {
                        player.end_turn();
                    }
                }
            }
        }
        else if instruction.starts_with("use_item") {
            let parts: Vec<&str> = instruction.split_whitespace().collect();
            if parts.len() == 2 {
                self.use_item(&mut player, &mut alien, parts[1].to_string(), None, &mut map);
            } else if parts.len() == 3 {
                if let Ok(room_index) = parts[2].parse::<usize>() {
                    self.use_item(&mut player, &mut alien, parts[1].to_string(), Some(room_index), &mut map);
                }
            }
            player.decrease_remaining_actions(1); // Remove 1 act by using item
        }
      
        else if instruction.starts_with("pick_up_scrap") {
            let parts: Vec<&str> = instruction.split_whitespace().collect();
            if parts.len() == 2 {
                if let Ok(amount) = parts[1].parse::<i32>() {
                    let current_room_index = player.get_current_room_index();
                    if map.remove_scrap_from_room(current_room_index, amount) {
                        player.add_scrap(amount);
                        godot_print!("Picked up {} scrap tokens", amount);
                        player.decrease_remaining_actions(1);
                    } else {
                        godot_print!("Not enough scrap tokens in the room");
                    }
                }
            }
        }
        else if instruction.starts_with("pick_up_item") {
            let parts: Vec<&str> = instruction.split_whitespace().collect();
            if parts.len() == 2 {
                let item_name = parts[1];
                let current_room_index = player.get_current_room_index();
                let room_item = map.get_room_objects(current_room_index);
                if let Some(item) = room_item.iter().find(|item| item.name == item_name) {
                 
                    let room_limitation_name = item.room_limitation_name.clone();
                    player.add_item(item_name.to_string(), 1, room_limitation_name);
          
                    player.decrease_remaining_actions(1);
                    map.remove_object_from_room(current_room_index, item_name);
                }
            }
        }


        "continue".to_string()
    }

    fn use_item(&mut self, player: &mut GdMut<'_, Player>, alien: &mut GdMut<'_, Player>, item: String, room: Option<usize>, map: &GdMut<GameMap>) {
        if player.get_remaining_actions() > 0 {
            if let Some(item_dict) = player.get_item_slots().iter().find(|dict| dict.get("name").expect("Item name not found").to_string() == item) {
                let uses = item_dict.get("uses").expect("Item uses not found").to::<i32>();
                let room_limitation_name = item_dict.get("room_limitation_name").expect("Room limitation name not found").to::<String>();
                let current_room_index = player.get_current_room_index();
                let current_room_name = map.get_room_name(current_room_index);

                match item.as_str() {
                    "flamethrower" => {
                        alien.move_to_room(0); // Move alien to original spot
                        godot_print!("Used flamethrower on alien!");
                    }
                    "flare" => {
                        if let Some(room_index) = room {
                            alien.move_to_room(room_index.try_into().unwrap());
                            godot_print!("Used flare in room {}", room_index);
                        }
                    }
                    _ => godot_print!("Unknown item"),
                }

                if uses > 1 {
                    let new_uses = uses - 1;
                    player.drop_item(item.clone());
                    player.add_item(item.clone(), new_uses, room_limitation_name.clone());
                    godot_print!("Item uses left: {}", new_uses);
                } else {
                    player.drop_item(item.clone());
                    godot_print!("Item {} is out of uses and removed from inventory", item);
                }
                player.decrease_remaining_actions(1);

                // Check if the item used completes an objective
                let objectives = self.objectives.clone();
                for (index, objective) in objectives.iter().enumerate() {
                    if objective.bring_object == item && objective.place == current_room_name {
                        self.achieve_objective(index as i32);
                        godot_print!("Objective achieved: {}", objective.description);
                    }
                }
            } else {
                godot_print!("Item not found in inventory");
            }
        } else {
            godot_print!("Out of actions");
        }
    }

    // Moves the alien towards the player by the specified number of actions
    fn move_alien(&self, map: &GdMut<'_, GameMap>, player: &mut GdMut<'_, Player>, alien: &mut GdMut<'_, Player>, actions: i32) {

        let start = alien.get_current_room_index();
        let goal = player.get_current_room_index();

        // Perform BFS to find the shortest path
        let mut queue = VecDeque::new();
        let mut came_from = HashMap::new();
        queue.push_back(start);
        came_from.insert(start, None);

        while let Some(current) = queue.pop_front() {
            if current == goal {
                break;
            }

            for next in map.get_connected_rooms(current) {
                if !came_from.contains_key(&next) {
                    queue.push_back(next);
                    came_from.insert(next, Some(current));
                }
            }
        }

        // Reconstruct the path
        let mut path = Vec::new();
        let mut current = goal;
        while let Some(&Some(prev)) = came_from.get(&current) {
            path.push(current);
            current = prev;
        }
        path.push(start);
        path.reverse();

        // Move the alien along the path by the specified number of actions
        for _ in 0..actions {
            if let Some(next_room) = path.get(1) {
                alien.move_to_room(*next_room);
                path.remove(0);
            } else {
                break;
            }
        }
    }
    #[func]
    fn place_mission_objects(&self, mut map: Gd<GameMap>) {
        let mut map = map.bind_mut();
        for objective in &self.objectives {
            for spawn_object in &objective.objects_to_spawn {
                let item = Item {
                    name: spawn_object.object.clone(),
                    room_limitation_name: spawn_object.place.clone(),
                };
                
                map.add_object_to_room(&spawn_object.room, item);
            }
        }
    }

    #[func]
    fn parse_instruction(&self, map: Gd<GameMap>, instruction: String) -> String {
        let map = map.bind();
        if instruction == "end_turn" {
            "End this turn".to_string()
        } 
        
        else if instruction.starts_with("move_to") {
            let parts: Vec<&str> = instruction.split_whitespace().collect();
            if parts.len() == 2 {
                if let Ok(room_index) = parts[1].parse::<usize>() {
                    let room_name = map.get_room_name(room_index.try_into().unwrap());
                    return format!("Move to room \"{}\"", room_name);
                }
            }
            "Invalid move instruction".to_string()
        }
        
        else if instruction.starts_with("flee_to") {
            let parts: Vec<&str> = instruction.split_whitespace().collect();
            if parts.len() == 2 {
                if let Ok(room_index) = parts[1].parse::<usize>() {
                    let room_name = map.get_room_name(room_index.try_into().unwrap());
                    return format!("Flee to room \"{}\" [Will hurt morale]", room_name);
                }
            }
            "Invalid move instruction".to_string()
        }

        else if instruction.starts_with("use_item") {
            let parts: Vec<&str> = instruction.split_whitespace().collect();
            if parts.len() == 2 {
                if parts[1] == "flamethrower" {
                    return "Shoot the \"Flamethrower\" [Will scare the creature back to its lair]".to_string();
                } else {
                    return format!("Use item \"{}\"", parts[1]);
                }
            } else if parts.len() == 3 {
                if parts[1] == "flare" {
                    let room_index = parts[2].parse::<usize>().unwrap();
                    let room_name = map.get_room_name(room_index.try_into().unwrap());
                    return format!("Use item \"Flare\" to room \"{}\"", room_name);
                }
            }
            "Invalid item instruction".to_string()
        }
        
        else if instruction.starts_with("pick_up_scrap") {
            let parts: Vec<&str> = instruction.split_whitespace().collect();
            if parts.len() == 2 {
                if let Ok(amount) = parts[1].parse::<i32>() {
                    return format!("Pick up {} scrap tokens", amount);
                }
            }
            "Invalid pick up scrap instruction".to_string()
        }
        
        else if instruction.starts_with("pick_up_item") {
            let parts: Vec<&str> = instruction.split_whitespace().collect();
            if parts.len() == 2 {
                return format!("Pick up item \"{}\"", parts[1]);
            }
            "Invalid pick up item instruction".to_string()
        }
        else {
            format!("Unknown instruction \"{}\" ", instruction )
        }
    }

    #[func]
    fn achieve_objective(&mut self, index: i32) {
        if let Some(objective) = self.objectives.get_mut(index as usize) {
            objective.achieved = true;
            godot_print!("Objective achieved: {}", objective.description);
        }
    }

    #[func]
    fn get_objectives_text(&self) -> String {
        let mut objectives_text = String::new();
        for objective in &self.objectives {
            let status = if objective.achieved { "✓" } else { "✗" };
            let objective_str = format!(
                "{} {}\n",
                status, objective.description
            );
            objectives_text.push_str(&objective_str);
        }
        objectives_text
    }

}