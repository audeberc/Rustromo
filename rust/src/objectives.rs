use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct Objective {
    pub place: String,
    pub description: String,
    pub bring_object: String,
    pub objects_to_spawn: Vec<SpawnObject>,
    pub action: String,
    pub achieved: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SpawnObject {
    pub room: String,
    pub object: String,
    pub place: String, // Add place field
}

#[derive(Debug, Deserialize)]
pub struct Objectives {
    pub objectives: Vec<Objective>,
}

impl Objectives {
    pub fn new() -> Self {
        let objectives = vec![
            Objective {
                place: "Engines Room".to_string(),
                description: "Fix the engine room with the wrench from storage".to_string(),
                bring_object: "wrench".to_string(),
                objects_to_spawn: vec![
                    SpawnObject {
                        room: "Storage".to_string(),
                        object: "wrench".to_string(),
                        place: "Engines room".to_string(), 
                    },
                ],
                action: "fix".to_string(),
                achieved: false,
            },
            Objective {
                place: "Computer Room".to_string(),
                description: "Reboot the Computer  with the keycard from Barracks".to_string(),
                bring_object: "keycard".to_string(),
                objects_to_spawn: vec![
                    SpawnObject {
                        room: "Barracks".to_string(),
                        object: "keycard".to_string(),
                        place: "Computer Room".to_string(),
                    },
                ],
                action: "reboot".to_string(),
                achieved: false,
            },
        ];
        Self { objectives }
        
    }

    pub fn mark_as_achieved(&mut self, index: usize) {
        if let Some(objective) = self.objectives.get_mut(index) {
            objective.achieved = true;
        }
    }
}