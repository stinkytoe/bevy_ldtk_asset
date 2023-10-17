use crate::ldtk_json;
use crate::world::World;

use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
};
// use serde::Deserialize;
use std::collections::HashMap;

use super::ldtk_level::LdtkLevel;

#[derive(Debug, TypePath, TypeUuid)]
#[uuid = "e131e69d-f619-4fec-9fa5-71fef82f9c81"]
pub struct LdtkProject {
    pub bg_color: Color,
    pub level_backgrounds: HashMap<String, Handle<Image>>,
    pub level_file_handles: HashMap<String, Handle<LdtkLevel>>,
    pub tilesets: HashMap<i64, Handle<Image>>,
    pub value: ldtk_json::LdtkJson,
    pub worlds: HashMap<String, World>,
}
