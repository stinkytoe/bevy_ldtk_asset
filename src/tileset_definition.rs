use bevy_asset::{Asset, Handle, LoadContext};
use bevy_math::{I64Vec2, Vec2};
use bevy_reflect::Reflect;
use bevy_render::texture::Image;

use crate::label::ProjectAssetPath;
use crate::ldtk;
use crate::ldtk_path::ldtk_path_to_bevy_path;
use crate::project_loader::ProjectContext;
use crate::uid::Uid;
use crate::Result;

#[derive(Debug, Reflect)]
pub struct TileCustomMetadata {
    pub data: String,
    pub tile_id: Uid,
}

impl TileCustomMetadata {
    pub(crate) fn new(value: &ldtk::TileCustomMetadata) -> Self {
        Self {
            data: value.data.clone(),
            tile_id: value.tile_id,
        }
    }
}

#[derive(Debug, Reflect)]
pub struct EnumTagValue {
    pub enum_value_id: String,
    pub tile_ids: Vec<i64>,
}

impl EnumTagValue {
    pub(crate) fn new(value: &ldtk::EnumTagValue) -> Self {
        Self {
            enum_value_id: value.enum_value_id.clone(),
            tile_ids: value.tile_ids.clone(),
        }
    }
}

#[derive(Asset, Debug, Reflect)]
pub struct TilesetDefinition {
    pub cell_size: I64Vec2,
    pub custom_data: Vec<TileCustomMetadata>,
    pub enum_tags: Vec<EnumTagValue>,
    pub identifier: String,
    pub padding: i64,
    pub size: Vec2,
    pub tileset_image: Option<Handle<Image>>,
    pub tags: Vec<String>,
    pub tags_source_enum_uid: Option<i64>,
    pub tile_grid_size: i64,
}

impl TilesetDefinition {
    pub(crate) fn create_handle_pair(
        value: &ldtk::TilesetDefinition,
        project_asset_path: &ProjectAssetPath,
        load_context: &mut LoadContext,
        project_context: &ProjectContext,
    ) -> Result<(Uid, Handle<Self>)> {
        let identifier = value.identifier.clone();
        let uid = value.uid;

        let tileset_definition_asset_path =
            project_asset_path.to_tileset_definition_asset_path(&identifier);

        let cell_size = (value.c_wid, value.c_hei).into();
        let custom_data = value
            .custom_data
            .iter()
            .map(TileCustomMetadata::new)
            .collect();
        let enum_tags = value.enum_tags.iter().map(EnumTagValue::new).collect();
        let padding = value.padding;
        let size = (value.px_wid as f32, value.px_hei as f32).into();
        let tileset_image = value.rel_path.as_ref().map(|rel_path| {
            let bevy_path = ldtk_path_to_bevy_path(project_context.project_directory, rel_path);
            load_context.load(bevy_path)
        });
        let tags = value.tags.clone();
        let tags_source_enum_uid = value.tags_source_enum_uid;
        let tile_grid_size = value.tile_grid_size;

        let tileset_definition = TilesetDefinition {
            identifier,
            cell_size,
            custom_data,
            enum_tags,
            padding,
            size,
            tileset_image,
            tags,
            tags_source_enum_uid,
            tile_grid_size,
        }
        .into();

        let handle = load_context.add_loaded_labeled_asset(
            tileset_definition_asset_path.to_asset_label(),
            tileset_definition,
        );

        Ok((uid, handle))
    }
}
