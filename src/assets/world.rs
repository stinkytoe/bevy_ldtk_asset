use bevy::prelude::*;

use crate::ldtk;

#[derive(Asset, Clone, Debug, TypePath)]
pub struct WorldAsset {
    identifier: String,
}

impl From<ldtk::LdtkJson> for WorldAsset {
    fn from(_value: ldtk::LdtkJson) -> Self {
        Self {
            identifier: "World".to_string(),
        }
    }
}

impl From<ldtk::World> for WorldAsset {
    fn from(value: ldtk::World) -> Self {
        Self {
            identifier: value.identifier.clone(),
        }
    }
}

impl WorldAsset {
    pub fn identifier(&self) -> &String {
        &self.identifier
    }
}
