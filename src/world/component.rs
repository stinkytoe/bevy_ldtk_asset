use bevy::{prelude::*, utils::HashMap};

use crate::layer::LayersToLoad;

#[derive(Component, Debug, Default, Reflect)]
pub struct WorldComponent {}

#[derive(Clone, Component, Debug, Reflect)]
pub enum LevelsToLoad {
    None,
    ByIdentifiers(HashMap<String, LayersToLoad>),
    ByIids(HashMap<String, LayersToLoad>),
    All(LayersToLoad),
}

impl Default for LevelsToLoad {
    fn default() -> Self {
        Self::All(LayersToLoad::default())
    }
}
