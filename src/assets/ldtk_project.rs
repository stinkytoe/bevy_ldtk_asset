use crate::ldtk_json;
use crate::world::World;
use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
};
// use serde::Deserialize;
use super::ldtk_level::LdtkLevel;
use std::collections::HashMap;

#[derive(Debug, TypePath, TypeUuid)]
#[uuid = "e131e69d-f619-4fec-9fa5-71fef82f9c81"]
pub(crate) struct LdtkProject {
    pub(crate) _bg_color: Color,
    pub(crate) _level_backgrounds: HashMap<String, Handle<Image>>,
    pub(crate) _level_file_handles: HashMap<String, Handle<LdtkLevel>>,
    pub(crate) _tilesets: HashMap<i64, Handle<Image>>,
    pub(crate) _value: ldtk_json::LdtkJson,
    pub(crate) _worlds: HashMap<String, World>,
}
