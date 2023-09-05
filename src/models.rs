use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum Status {
    Todo,
    InProgress,
    Closed,
}

#[derive(Deserialize, Debug)]
pub struct Epic {
    name: String,
    description: String,
    status: Status,
    stories: Vec<usize>,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        Epic {
            name,
            description,
            status: Status::Todo,
            stories: Vec::new(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Story {
    name: String,
    description: String,
    status: Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        Story {
            name,
            description,
            status: Status::Todo,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct DBState {
    last_item_id: usize,
    epics: HashMap<String, Epic>,
    stories: HashMap<String, Story>,
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
