use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::world::LevelsToLoad;

#[derive(Clone, Component, Debug, Reflect)]
pub enum WorldsToLoad {
    None,
    ByIdentifiers(HashMap<String, LevelsToLoad>),
    ByIids(HashMap<String, LevelsToLoad>),
    All(LevelsToLoad),
}

impl Default for WorldsToLoad {
    fn default() -> Self {
        Self::All(LevelsToLoad::default())
    }
}
