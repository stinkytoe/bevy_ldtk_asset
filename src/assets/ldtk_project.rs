use crate::assets::structs::world::World;
use crate::ldtk_json;
use bevy::{prelude::*, reflect::TypePath};
use std::collections::HashMap;

#[derive(Asset, Debug, TypePath)]
pub struct LdtkProject {
    pub(crate) value: ldtk_json::LdtkJson,
    pub(crate) _worlds: HashMap<String, World>,
}

impl LdtkProject {
    pub fn json(&self) -> &ldtk_json::LdtkJson {
        &self.value
    }
}
