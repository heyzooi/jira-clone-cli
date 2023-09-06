use std::{collections::HashMap, fmt::Display};
use anyhow::{anyhow, Result};
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    NavigateToEpicDetail { epic_id: u32 },
    NavigateToStoryDetail { epic_id: u32, story_id: u32 },
    NavigateToPreviousPage,
    CreateEpic,
    UpdateEpicStatus { epic_id: u32 },
    DeleteEpic { epic_id: u32 },
    CreateStory { epic_id: u32 },
    UpdateStoryStatus { story_id: u32 },
    DeleteStory { epic_id: u32, story_id: u32 },
    Exit,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Open       => write!(f, "Open"),
            Status::InProgress => write!(f, "In Progress"),
            Status::Resolved   => write!(f, "Resolved"),
            Status::Closed     => write!(f, "Closed"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Epic {
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<u32>,
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct DBState {
    pub last_item_id: u32,
    pub epics: HashMap<u32, Epic>,
    pub stories: HashMap<u32, Story>,
}

impl DBState {
    fn new() -> Self {
        DBState {
            last_item_id: 0,
            epics: HashMap::new(),
            stories: HashMap::new(),
        }
    }

    fn next_id(&self) -> u32 {
        self.last_item_id + 1
    }

    fn generate_new_id(&mut self) -> u32 {
        self.last_item_id = self.next_id();
        self.last_item_id
    }

    pub fn create_epic(&mut self, epic: Epic) -> Result<u32> {
        let id = self.generate_new_id();
        self.epics.insert(id, epic);
        Ok(id)
    }

    pub fn delete_epic(&mut self, epic_id: &u32) -> Result<Epic> {
        let epic = self.epics.remove(epic_id).ok_or(anyhow!("Epic {epic_id} not found"))?;
        for story_id in &epic.stories {
            self.stories.remove(story_id);
        }
        Ok(epic)
    }

    pub fn update_epic_status(&mut self, epic_id: &u32, status: Status) -> Result<&Epic> {
        let epic = self.epics.get_mut(epic_id).ok_or(anyhow!("Epic {epic_id} not found"))?;
        epic.status = status;
        Ok(epic)
    }

    pub fn create_story(&mut self, epic_id: &u32, story: Story) -> Result<u32> {
        let id = self.next_id();
        let epic = self.epics.get_mut(epic_id).ok_or(anyhow!("Epic {epic_id} not found"))?;
        epic.stories.push(id);
        self.stories.insert(id, story);
        assert_eq!(id, self.generate_new_id());
        Ok(id)
    }

    pub fn delete_story(&mut self, epic_id: &u32, story_id: &u32) -> Result<Story> {
        let epic = self.epics.get_mut(epic_id).ok_or(anyhow!("Epic {epic_id} not found"))?;
        epic.stories.sort();
        let index = epic.stories.binary_search(story_id).map_err(|_| anyhow!("Story {story_id} not found"))?;
        epic.stories.remove(index);
        self.stories.remove(story_id).ok_or(anyhow!("Story {story_id} not found"))
    }
    
    pub fn update_story_status(&mut self, story_id: &u32, status: Status) -> Result<&Story> {
        let story = self.stories.get_mut(story_id).ok_or(anyhow!("Story {story_id} not found"))?;
        story.status = status;
        Ok(story)
    }

    pub fn get_story_by_id(&self, story_id: &u32) -> Result<&Story> {
        self.stories.get(story_id).ok_or(anyhow!("Story {story_id} not found"))
    }

    pub fn get_epic_by_id(&self, epic_id: &u32) -> Result<&Epic> {
        self.epics.get(epic_id).ok_or(anyhow!("Epic {epic_id} not found"))
    }
}
