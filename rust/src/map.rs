use godot::prelude::*;

#[derive(Debug, Clone)]
struct Room {
    name: String,
    description: String,
    x: f32,
    y: f32,
    scrap_tokens: i32, // Add scrap tokens to the room
    objects: Vec<Item>, // Change objects to store Item instead of String
}

#[derive(Debug, Clone)]
pub struct Item { // Make Item public
    pub name: String, // Make fields public
    pub room_limitation_name: String, // Make fields public
}

#[derive(Debug, Clone)]
struct Edge {
    room_a: usize, 
    room_b: usize, 
}

#[derive(GodotClass)]
#[class(base = Node)]
pub struct GameMap {
    base: Base<Node>,
    rooms: Vec<Room>,  
    edges: Vec<Edge>, 
}

use godot::prelude::INode;
#[godot_api]
impl INode for GameMap {
    fn init(base: Base<Node>) -> Self {
        godot_print!("GameMap initialized");
        Self {
            base,
            rooms: Vec::new(),
            edges: Vec::new(),
        }
    }
}

#[godot_api]
impl GameMap {
    // Adds a new room and returns its index
    #[func]
pub fn add_room(&mut self, name: String, description: String, x: f32, y: f32) -> i32 {
    self.rooms.push(Room { name, description, x, y, scrap_tokens: 0, objects: Vec::new() });
    (self.rooms.len() - 1) as i32
}

// Connects two rooms
#[func]
pub fn connect_rooms(&mut self, index_a: i32, index_b: i32) {
    self.edges.push(Edge {
        room_a: index_a as usize,
        room_b: index_b as usize,
    });
}

// Gets information about a room
#[func]
pub fn get_room_info(&self, index: i32) -> String {
    if let Some(room) = self.rooms.get(index as usize) {
        format!("Room: {} [{}]\nScrap Tokens in room: {}", room.name, room.description, room.scrap_tokens)
    } else {
        "Room not found".to_string()
    }
}

// Gets room name
#[func]
pub fn get_room_name(&self, index: i32) -> String {
    if let Some(room) = self.rooms.get(index as usize) {
        format!("{}", room.name)
    } else {
        "Room not found".to_string()
    }
}

// Checks if two rooms are connected
#[func]
pub fn are_rooms_connected(&self, index_a: i32, index_b: i32) -> bool {
    self.edges.iter().any(|edge| {
        (edge.room_a == index_a as usize && edge.room_b == index_b as usize)
            || (edge.room_a == index_b as usize && edge.room_b == index_a as usize)
    })
}

// Gets the connected rooms to a given room
#[func]
pub fn get_connected_rooms(&self, index: i32) -> Vec<i32> {
    let mut connected_rooms = Vec::new();
    for edge in &self.edges {
        if edge.room_a == index as usize {
            connected_rooms.push(edge.room_b as i32);
        } else if edge.room_b == index as usize {
            connected_rooms.push(edge.room_a as i32);
        }
    }
    connected_rooms
    }

    // Gets the coordinates of a room
    #[func]
    pub fn get_room_coordinates(&self, index: i32) -> Vector2 {
        if let Some(room) = self.rooms.get(index as usize) {
            Vector2::new(room.x, room.y)
        } else {
            Vector2::ZERO
        }
    }

    pub fn get_rooms_within_distance(&self, start_room: usize, min_distance: usize, max_distance: usize) -> Vec<usize> {
        let mut visited = vec![false; self.rooms.len()];
        let mut queue = std::collections::VecDeque::new();
        let mut result = Vec::new();

        queue.push_back((start_room, 0));
        visited[start_room] = true;

        while let Some((current_room, current_distance)) = queue.pop_front() {
            if current_distance >= min_distance && current_distance <= max_distance {
                result.push(current_room);
            }
            if current_distance < max_distance {
                for neighbor in self.get_connected_rooms(current_room as i32) {
                    if !visited[neighbor as usize] {
                        visited[neighbor as usize] = true;
                        queue.push_back((neighbor as usize, current_distance + 1));
                    }
                }
            }
        }

        result
    }

    pub fn add_scrap_to_room(&mut self, index: i32, amount: i32) {
        if let Some(room) = self.rooms.get_mut(index as usize) {
            room.scrap_tokens += amount;
        }
    }

    pub fn remove_scrap_from_room(&mut self, index: i32, amount: i32) -> bool {
        if let Some(room) = self.rooms.get_mut(index as usize) {
            if room.scrap_tokens >= amount {
                room.scrap_tokens -= amount;
                return true;
            }
        }
        false
    }

    pub fn get_scrap_tokens_in_room(&self, index: i32) -> i32 {
        if let Some(room) = self.rooms.get(index as usize) {
            room.scrap_tokens
        } else {
            0
        }
    }

    pub fn add_object_to_room(&mut self, room_name: &str, item: Item) {
  
        if let Some(room) = self.rooms.iter_mut().find(|r| r.name == room_name) {
            room.objects.push(item);

        } else {
            godot_print!("Room not found");
        }
    }

    pub fn get_room_objects(&self, index: i32) -> Vec<Item> {
        if let Some(room) = self.rooms.get(index as usize) {
            room.objects.clone()
        } else {
            Vec::new()
        }
    }   

    pub fn remove_object_from_room(&mut self, index: i32, object_name: &str) -> bool {
        if let Some(room) = self.rooms.get_mut(index as usize) {
            if let Some(pos) = room.objects.iter().position(|item| item.name == object_name) {
                room.objects.remove(pos);
                return true;
            }
        }
        false}
}
