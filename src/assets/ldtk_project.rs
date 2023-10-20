use crate::ldtk_json;
use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
};
// use serde::Deserialize;
use super::ldtk_level::LdtkLevel;
use std::collections::HashMap;

pub type WorldIid = String;
pub type LevelIid = String;

#[derive(Debug, TypePath, TypeUuid)]
#[uuid = "e131e69d-f619-4fec-9fa5-71fef82f9c81"]
pub struct LdtkProject {
    pub(crate) _bg_color: Color, // do we need this?
    pub(crate) level_backgrounds: HashMap<LevelIid, Handle<Image>>,
    pub(crate) level_file_handles: HashMap<LevelIid, Handle<LdtkLevel>>,
    pub(crate) tilesets: HashMap<i64, Handle<Image>>,
    pub(crate) _value: ldtk_json::LdtkJson,
    pub(crate) world_level_map: HashMap<WorldIid, Vec<LevelIid>>,
    // pub(crate) _worlds: HashMap<String, World>,
}

impl LdtkProject {
    pub fn json(&self) -> &ldtk_json::LdtkJson {
        &self._value
    }

    pub fn get_world_iids(&self) -> impl Iterator<Item = &WorldIid> {
        self.world_level_map.keys()
    }

    pub fn get_level_iids(&self) -> impl Iterator<Item = &LevelIid> {
        self.world_level_map.values().flatten()
    }

    // pub(crate) fn get_levels(&self) -> impl Iterator<Item = &Level> {
    //     self._worlds
    //         .values()
    //         .flat_map(|world| world._levels.values())
    // }
}
