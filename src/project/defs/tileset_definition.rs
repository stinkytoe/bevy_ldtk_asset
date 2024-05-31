use std::path::PathBuf;

use bevy::{math::I64Vec2, prelude::*};

use crate::ldtk;
use crate::project::defs::EnumTagValue;
use crate::project::defs::TileCustomMetadata;

#[derive(Debug, Default, Reflect)]
pub struct TilesetDefinition {
    pub grid_size: I64Vec2,
    pub custom_data: Vec<TileCustomMetadata>,
    // embedAtlas not currently supported!
    pub enum_tags: Vec<EnumTagValue>,
    pub identifier: String,
    pub padding: i64,
    pub size: Vec2,
    pub rel_path: Option<PathBuf>,
    pub spacing: i64,
    pub tags: Vec<String>,
    pub tile_grid_size: i64,
    pub uid: i64,
}

impl From<&ldtk::TilesetDefinition> for TilesetDefinition {
    fn from(value: &ldtk::TilesetDefinition) -> Self {
        Self {
            grid_size: (value.c_wid, value.c_hei).into(),
            custom_data: value
                .custom_data
                .iter()
                .map(TileCustomMetadata::from)
                .collect(),
            enum_tags: value.enum_tags.iter().map(EnumTagValue::from).collect(),
            identifier: value.identifier.clone(),
            padding: value.padding,
            size: (value.px_wid as f32, value.px_hei as f32).into(),
            rel_path: value.rel_path.as_ref().map(PathBuf::from),
            spacing: value.spacing,
            tags: value.tags.clone(),
            tile_grid_size: value.tile_grid_size,
            uid: value.uid,
        }
    }
}
