use bevy::prelude::*;

use crate::ldtk;

#[derive(Debug, Default, Reflect)]
pub struct TileCustomMetadata {
    pub data: String,
    pub tile_id: i64,
}

impl From<&ldtk::TileCustomMetadata> for TileCustomMetadata {
    fn from(value: &ldtk::TileCustomMetadata) -> Self {
        Self {
            data: value.data.clone(),
            tile_id: value.tile_id,
        }
    }
}
