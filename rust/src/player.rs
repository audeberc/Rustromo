use godot::prelude::*;
use godot::classes::Node;

#[derive(GodotClass)]
#[class(base = Node)]
pub struct Player {
    base: Base<Node>,
    actions_per_turn: i32,
    remaining_actions: i32,
    item_slots: Vec<String>,
    scraps: i32,
    current_room_index: i32,
    morale: f32
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
            scraps: 0,
            current_room_index: 0,
            morale: 0.0
        }
    }
}


#[godot_api]
impl Player {
    #[func]
    pub fn perform_action(&mut self) -> bool {
        if self.remaining_actions > 0 {
            self.remaining_actions -= 1;
            godot_print!("Performed an action!");
            true
        } else {
            godot_print!("Out of actions");
            false
        }
    }

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
    pub fn end_turn(&mut self) {
        godot_print!("Reseting turn");
        self.remaining_actions = self.actions_per_turn;
     
    }

    #[func]
    pub fn add_item(&mut self, item: String) {
        self.item_slots.push(item);

    }

    #[func]
    pub fn add_scrap(&mut self, amount: i32) {
        self.scraps += amount;

    }

    #[func]
    pub fn move_to_room(&mut self, new_room_index: i32) {
        if self.remaining_actions > 0 {self.current_room_index = new_room_index;
            self.remaining_actions -= 1;}
        
   
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

    
}