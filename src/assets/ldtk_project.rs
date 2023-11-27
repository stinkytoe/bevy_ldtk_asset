use super::ldtk_level::LdtkLevel;
use crate::ldtk_json;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Asset, Debug, TypePath)]
pub struct LdtkProject {
	pub value: ldtk_json::LdtkJson,
	pub level_handle_map: HashMap<String, Handle<LdtkLevel>>,
}
