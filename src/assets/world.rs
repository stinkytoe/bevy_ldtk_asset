use std::path::PathBuf;

use bevy::prelude::*;

use crate::ldtk;

#[derive(Asset, Clone, Debug, TypePath)]
pub struct LdtkWorld {
    identifier: String,
}

impl From<ldtk::LdtkJson> for LdtkWorld {
    fn from(value: ldtk::LdtkJson) -> Self {
        Self {
            identifier: "World".to_string(),
        }
    }
}

impl From<ldtk::World> for LdtkWorld {
    fn from(value: ldtk::World) -> Self {
        error!("Ldtk Project is a multi world file, which is not currently supported!");
        unimplemented!()
    }
}

impl LdtkWorld {
    pub fn identifier(&self) -> &String {
        &self.identifier
    }
}
