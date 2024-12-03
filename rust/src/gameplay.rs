use godot::prelude::*;
use std::collections::{HashMap, VecDeque};

use crate::map::GameMap;
use crate::player::Player;

#[derive(GodotClass)]
#[class(base = Node)]
struct Gameplay {
    base: Base<Node>,
}

#[godot_api]
impl INode for Gameplay {
    fn init(base: Base<Node>) -> Self {
        godot_print!("Gameplay initialized");
        Self { base }
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
            let distant_rooms = map.get_rooms_within_distance(player.get_current_room_index() as usize, 3);
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
                let items = player.get_items();
                for (key, _) in items.iter_shared() {
                    if key.to_string() == "flamethrower" {
                        let alien_room = alien.get_current_room_index();
                        let player_room = player.get_current_room_index();
                        let distant_rooms = map.get_rooms_within_distance(player_room as usize, 2);

                        if distant_rooms.contains(&(alien_room as usize)) {
                            let instruction = format!("use_item {}", key.to_string());
                            movements.push(&instruction);
                        }
                    } else {
                        let instruction = format!("use_item {}", key.to_string());
                        movements.push(&instruction);
                    }
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
        info.push_str(&format!("Player's current room: {}\n", map.get_room_info(player.get_current_room_index())));
        info.push_str(&format!("Remaining actions: {}\n", player.get_remaining_actions()));
        info.push_str(&format!("Morale: {}\n", player.get_morale()));
        info
    }

    // Handles the selected item based on the instruction provided by Godot
    #[func]
    fn handle_selected_item(&self, map: Gd<GameMap>, mut player: Gd<Player>, mut alien: Gd<Player>, instruction: String) {
        let map = map.bind();
        let mut player = player.bind_mut();
        let mut alien = alien.bind_mut();

        if instruction == "end_turn" || player.get_remaining_actions() == 0 {
            // Handle end turn logic
            godot_print!("End Turn selected");

            player.end_turn(); // Reset actions for the next turn
            alien.end_turn();
            self.move_alien(map, player, alien, 1);
        } else if instruction.starts_with("move_to") {
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
                    player.move_to_room(room_index.try_into().unwrap());
                    let morale = player.get_morale() - 10.0;
                    player.set_morale(morale);
                    player.end_turn();
                }
            }
        }
        else if instruction.starts_with("use_item") {
            let parts: Vec<&str> = instruction.split_whitespace().collect();
            if parts.len() == 2 {
                self.use_item(player, alien, parts[1].to_string());
            }
        }


    }

    fn use_item(&self, mut player: GdMut<'_, Player>, mut alien: GdMut<'_, Player>, item: String) {
        if player.get_remaining_actions() > 0 {
            if let Some(uses) = player.get_item_uses_mut().get_mut(&item) {
                match item.as_str() {
                    "flamethrower" => {
                        alien.move_to_room(0); // Move alien to original spot
                        godot_print!("Used flamethrower on alien!");
                    }
                    "grapple_gun" => {
                        // Implement grapple gun logic here
                    }
                    _ => godot_print!("Unknown item"),
                }

                *uses -= 1;
                if *uses <= 0 {
                    player.get_item_slots_mut().retain(|i| i != &item);
                    godot_print!("Item {} is out of uses", item);
                }
                player.decrease_remaining_actions(1);
            } else {
                godot_print!("Item not found in inventory");
            }
        } else {
            godot_print!("Out of actions");
        }
    }

    // Moves the alien towards the player by the specified number of actions
    fn move_alien(&self, map: GdRef<'_, GameMap>, player: GdMut<'_, Player>, mut alien: GdMut<'_, Player>, actions: i32) {

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
                    return format!("Flee to room \"{}\"", room_name);
                }
            }
            "Invalid move instruction".to_string()
        }

        else if instruction.starts_with("use_item") {
            let parts: Vec<&str> = instruction.split_whitespace().collect();
            if parts.len() == 2 {
                return format!("Use item \"{}\"", parts[1]);
            }
            "Invalid item instruction".to_string()
            
        }
        
        else {
            "Unknown instruction".to_string()
        }
    }

}