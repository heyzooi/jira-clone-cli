use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Status {
    Open,
    InProgress,
    Closed,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Epic {
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<usize>,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        Epic {
            name,
            description,
            status: Status::Open,
            stories: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Story {
    pub name: String,
    pub description: String,
    pub status: Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        Story {
            name,
            description,
            status: Status::Open,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DBState {
    pub last_item_id: usize,
    pub epics: HashMap<i32, Epic>,
    pub stories: HashMap<i32, Story>,
}

impl DBState {
    fn new() -> Self {
        DBState {
            last_item_id: 0,
            epics: HashMap::new(),
            stories: HashMap::new(),
        }
    }

    fn generate_new_id(&mut self) -> usize {
        self.last_item_id += 1;
        self.last_item_id
    }
}
