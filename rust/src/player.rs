use godot::prelude::*;
use godot::classes::Node;
use crate::items::Item;

#[derive(GodotClass)]
#[class(base = Node)]
pub struct Player {
    base: Base<Node>,
    actions_per_turn: i32,
    remaining_actions: i32,
    item_slots: Vec<Item>,
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
            item_slots: Vec::new(),   
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
        self.remaining_actions = self.actions_per_turn;
    }

    #[func]
    pub fn add_item(&mut self, name: String, uses: i32, room_limitation_name: String) {
        if self.item_slots.len() < 3 {
            let item = Item::new(name, uses, room_limitation_name);
            self.item_slots.push(item);
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
    pub fn drop_item(&mut self, item_name: String) {
        if let Some(pos) = self.item_slots.iter().position(|x| x.name == item_name) {
            let removed_item = self.item_slots.remove(pos);
            godot_print!("Dropped item: {}", removed_item.name);
        } else {
            godot_print!("Item not found in inventory");
        }
    }

    #[func]
    pub fn get_item_slots(&self) -> Vec<Dictionary> {
        self.item_slots.iter().map(|item| {
            let mut dict = Dictionary::new();
            dict.insert("name", item.name.to_variant());
            dict.insert("uses", item.uses.to_variant());
            dict.insert("room_limitation_name", item.room_limitation_name.to_variant());
            dict
        }).collect()
    }

    pub fn decrease_remaining_actions(&mut self, amount: i32) {
        self.remaining_actions -= amount;
    }
}