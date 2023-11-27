use crate::ldtk_json;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use serde::Deserialize;

#[derive(Asset, TypePath, Debug, Deserialize)]
pub struct LdtkLevel {
	pub value: ldtk_json::Level,
}
