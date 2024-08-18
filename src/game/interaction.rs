use bevy::prelude::*;
use serde::{self, ser::SerializeStruct};


//interactable object component
#[derive(Component, Debug)]
pub struct Interactable {
    pub boundary: Rect,

    pub valid_directions: Vec<Facing>,
    pub interaction_count: u32,

    pub action: Vec<String>,
    pub dependancies: Vec<Entity>,
}

impl Interactable {
    pub fn new(boundary: Rect, valid_directions: Vec<Facing>) -> Self {
        Interactable {
            boundary,
            valid_directions,
            interaction_count: 0,
            action: Vec::new(),
            dependancies: Vec::new(),
        }
    }

    pub fn interact(&mut self) {
        self.interaction_count += 1;
    }
    
    pub fn add_dependancy(&mut self, entity: Entity) {
        self.dependancies.push(entity);
    }

    pub fn clear_dependancy(&mut self, entity: Entity) {
        self.dependancies.retain(|&x| x != entity);
    }

    pub fn has_dependancies(&self) -> bool {
        !self.dependancies.is_empty()
    }

}

impl serde::Serialize for Interactable {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Interactable", 5)?;
        state.serialize_field("boundary", &self.boundary)?;
        state.serialize_field("valid_directions", &self.valid_directions)?;
        state.serialize_field("interaction_count", &self.interaction_count)?;
        state.serialize_field("action", &self.action)?;
        state.serialize_field("dependancies", &self.dependancies)?;
        state.end()
    }
}

#[derive(Component, Debug)]
pub enum Facing {
    Up,
    Down,
    Left,
    Right,
}
impl serde::Serialize for Facing {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Facing::Up => serializer.serialize_str("Up"),
            Facing::Down => serializer.serialize_str("Down"),
            Facing::Left => serializer.serialize_str("Left"),
            Facing::Right => serializer.serialize_str("Right"),
        }
    }
}   