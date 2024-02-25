use bevy::prelude::*;

use crate::ldtk;

#[derive(Asset, Clone, Debug, TypePath)]
pub struct LevelAsset {
    identifier: String,
}

impl From<ldtk::Level> for LevelAsset {
    fn from(value: ldtk::Level) -> Self {
        Self {
            identifier: value.identifier.clone(),
        }
    }
}

impl LevelAsset {
    pub fn identifier(&self) -> &String {
        &self.identifier
    }
}
