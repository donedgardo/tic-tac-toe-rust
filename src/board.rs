use std::collections::HashMap;
use crate::play_markers::PlayMarkers;

pub struct Board {
    pub(crate) spaces: HashMap<u8, PlayMarkers>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            spaces: HashMap::new()
        }
    }

    pub fn copy(&self) -> Self {
        Self {
            spaces: self.spaces.clone(),
        }
    }

    pub fn play(&mut self, space: u8, marker: &PlayMarkers) {
        self.spaces.insert(space, *marker);
    }

    pub fn is_full(&self) -> bool {
        self.spaces.len() == 9
    }

    pub fn is_space_played(&self, space: &u8) -> bool {
        self.spaces.contains_key(&space)
    }

    pub fn get_space_marker(&self, space: &u8) -> Option<&PlayMarkers> {
        self.spaces.get(space)
    }

}
