use std::collections::HashMap;

pub struct Item {
    pub name: String,
    pub uses: i32,
}

pub fn get_items() -> HashMap<String, Item> {
    let mut items = HashMap::new();
    items.insert("flamethrower".to_string(), Item { name: "flamethrower".to_string(), uses: 3 });
    items.insert("grapple_gun".to_string(), Item { name: "grapple_gun".to_string(), uses: 5 });
    items
}
