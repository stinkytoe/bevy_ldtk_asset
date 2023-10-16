use crate::ldtk_json;
use crate::world::World;

use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
};
// use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, TypePath, TypeUuid)]
#[uuid = "e131e69d-f619-4fec-9fa5-71fef82f9c81"]
pub struct LdtkProject {
    pub bg_color: Color,
    pub defs: ldtk_json::Definitions,
    pub external_levels: bool,
    pub iid: String,
    pub json_version: String,
    pub worlds: HashMap<String, World>,
    pub level_backgrounds: HashMap<String, Handle<Image>>,
}
