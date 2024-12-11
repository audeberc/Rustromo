#[derive(Debug, Clone)]
pub struct Item {
    pub name: String,
    pub uses: i32,
    pub room_limitation_name: String, 
}

impl Item {
    pub fn new(name: String, uses: i32, room_limitation_name: String) -> Self {
        Self {
            name,
            uses,
            room_limitation_name,
        }
    }
}