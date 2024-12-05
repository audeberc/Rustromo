use godot::prelude::*;
use godot::classes::Node;
use std::collections::HashMap;

#[derive(GodotClass)]
#[class(base = Node)]
pub struct Player {
    base: Base<Node>,
    actions_per_turn: i32,
    remaining_actions: i32,
    item_slots: Vec<String>,
    item_uses: HashMap<String, i32>,
    scraps: i32,
    current_room_index: i32,
    morale: f32,
}

use godot::prelude::INode;

#[godot_api]
impl INode for Player {
    fn init(base: Base<Node>) -> Self {
        godot_print!("Player initialized");
        Self {
            base,
            actions_per_turn: 0,
            remaining_actions: 0,
            item_slots: Vec::new(),    // No items initially
            item_uses: HashMap::new(),
            scraps: 0,
            current_room_index: 0,
            morale: 0.0,
        }
    }
}

#[godot_api]
impl Player {
    #[func]
    pub fn initialize(&mut self, actions_per_turn: i32, scraps: i32, current_room_index: i32, morale: f32) {
        self.actions_per_turn = actions_per_turn;
        self.remaining_actions = actions_per_turn;
        self.scraps = scraps;
        self.current_room_index = current_room_index;
        self.morale = morale;
    }

    #[func]
    pub fn get_remaining_actions(&self) -> i32 {
        self.remaining_actions
    }

    #[func]
    pub fn get_scraps(&self) -> i32 {
        self.scraps
    }


    #[func]
    pub fn end_turn(&mut self) {
        godot_print!("Reseting turn");
        self.remaining_actions = self.actions_per_turn;
    }

    #[func]
    pub fn add_item(&mut self, item: String, uses: i32) {
        if self.item_slots.len() < 3 {
            self.item_slots.push(item.clone());
            self.item_uses.insert(item, uses);
        } else {
            godot_print!("Cannot carry more than 3 items");
        }
    }

    #[func]
    pub fn add_scrap(&mut self, amount: i32) {
        self.scraps += amount;
    }

    #[func]
    pub fn move_to_room(&mut self, new_room_index: i32) {
        if self.remaining_actions > 0 {
            self.current_room_index = new_room_index;
            self.remaining_actions -= 1;
        }
    }

    #[func]
    pub fn get_current_room_index(&self) -> i32 {
        self.current_room_index
    }

    #[func]
    pub fn get_morale(&self) -> f32 {
        self.morale
    }

    #[func]
    pub fn set_morale(&mut self, new_morale: f32) {
        self.morale = new_morale;
    }

    #[func]
    pub fn drop_item(&mut self, item: String) {
        if let Some(pos) = self.item_slots.iter().position(|x| *x == item) {
            self.item_slots.remove(pos);
            self.item_uses.remove(&item);
            godot_print!("Dropped item: {}", item);
        } else {
            godot_print!("Item not found in inventory");
        }
    }

    #[func]
    pub fn get_items(&self) -> Dictionary {
        let mut dict = Dictionary::new();
        for (item, uses) in self.item_uses.iter() {
            dict.insert(item.to_string(), *uses);
        }
        dict
    }

    #[func]
    pub fn get_item_slots(&self) -> PackedStringArray {
        let mut array = PackedStringArray::new();
        for item in self.item_slots.iter() {
            array.push(item);
        }
        array
    }

    pub fn get_item_uses_mut(&mut self) -> &mut HashMap<String, i32> {
        &mut self.item_uses
    }

    pub fn get_item_slots_mut(&mut self) -> &mut Vec<String> {
        &mut self.item_slots
    }

    pub fn decrease_remaining_actions(&mut self, amount: i32) {
        self.remaining_actions -= amount;
    }
}