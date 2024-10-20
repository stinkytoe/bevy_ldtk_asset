use bevy::math::{I64Vec2, Vec2};
use bevy::reflect::Reflect;

use crate::ldtk;
use crate::uid::Uid;

#[derive(Debug, Reflect)]
pub struct TileCustomMetadata {
    pub data: String,
    pub tile_id: i64,
}

impl From<ldtk::TileCustomMetadata> for TileCustomMetadata {
    fn from(value: ldtk::TileCustomMetadata) -> Self {
        Self {
            data: value.data,
            tile_id: value.tile_id,
        }
    }
}

#[derive(Debug, Reflect)]
pub struct EnumTagValue {
    pub enum_value_id: String,
    pub tile_ids: Vec<i64>,
}

impl From<ldtk::EnumTagValue> for EnumTagValue {
    fn from(value: ldtk::EnumTagValue) -> Self {
        Self {
            enum_value_id: value.enum_value_id,
            tile_ids: value.tile_ids,
        }
    }
}

#[derive(Debug, Reflect)]
pub struct TilesetDefinition {
    pub cell_size: I64Vec2,
    pub custom_data: Vec<TileCustomMetadata>,
    pub enum_tags: Vec<EnumTagValue>,
    pub identifier: String,
    pub padding: i64,
    pub size: Vec2,
    pub rel_path: Option<String>,
    pub tags: Vec<String>,
    pub tags_source_enum_uid: Option<i64>,
    pub tile_grid_size: i64,
}

impl From<ldtk::TilesetDefinition> for (Uid, TilesetDefinition) {
    fn from(value: ldtk::TilesetDefinition) -> Self {
        let uid = value.uid;

        let cell_size = (value.c_wid, value.c_hei).into();
        let custom_data = value
            .custom_data
            .into_iter()
            .map(TileCustomMetadata::from)
            .collect();
        let enum_tags = value
            .enum_tags
            .into_iter()
            .map(EnumTagValue::from)
            .collect();
        let identifier = value.identifier;
        let padding = value.padding;
        let size = (value.px_wid as f32, value.px_hei as f32).into();
        let rel_path = value.rel_path;
        let tags = value.tags;
        let tags_source_enum_uid = value.tags_source_enum_uid;
        let tile_grid_size = value.tile_grid_size;

        (
            uid,
            TilesetDefinition {
                cell_size,
                custom_data,
                enum_tags,
                identifier,
                padding,
                size,
                rel_path,
                tags,
                tags_source_enum_uid,
                tile_grid_size,
            },
        )
    }
}
