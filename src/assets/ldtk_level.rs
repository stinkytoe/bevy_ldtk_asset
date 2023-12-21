use std::path::PathBuf;

use crate::ldtk_json;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use serde::Deserialize;

#[derive(Asset, TypePath, Debug, Deserialize)]
pub struct LdtkLevel {
    pub value: ldtk_json::Level,
    pub dir: PathBuf,
}

impl LdtkLevel {
    pub fn new(value: ldtk_json::Level, dir: PathBuf) -> Self {
        debug!("loaded LdtkLevel with dir: {dir:?}");
        // debug!(
        // 	"for fun: {:?}",
        // 	dir.join("../").join("../basic_tileset_and_assets_standard")
        // );
        Self { value, dir }
    }
}
