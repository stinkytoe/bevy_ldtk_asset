#![allow(missing_docs)]

use bevy_asset::{Asset, Handle, LoadContext};
use bevy_image::Image;
use bevy_math::I64Vec2;
use bevy_reflect::Reflect;

use crate::Result;
use crate::asset_labels::ProjectAssetPath;
use crate::ldtk_asset_trait::LdtkAssetWithTags;
use crate::ldtk_path::ldtk_path_to_bevy_path;
use crate::project_loader::ProjectContext;
use crate::uid::Uid;
use crate::{ldtk, ldtk_import_error};

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

/// A tileset definition for use in visualizations.
///
/// See [TilesetDefinition](https://ldtk.io/json/#ldtk-TilesetDefJson)
#[derive(Asset, Debug, Reflect)]
pub struct TilesetDefinition {
    pub cell_size: I64Vec2,
    pub custom_data: Vec<TileCustomMetadata>,
    pub enum_tags: Vec<EnumTagValue>,
    pub identifier: String,
    pub padding: i64,
    pub size: I64Vec2,
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
        // see https://github.com/stinkytoe/bevy_ldtk_asset/issues/35
        if value.embed_atlas.is_some() {
            return Err(ldtk_import_error!(
                "This LDtk project, {project_asset_path:?}, contains an embedded atlas!\
                 Licensing prevents us from loading this asset. At this time we are considering\
                 it to be an error to attempt to load an LDtk project with an embedded atlas!\
                 See https://github.com/stinkytoe/bevy_ldtk_asset/issues/35 for a discussion\
                 relating to this decision."
            ));
        }

        let identifier = value.identifier.clone();
        let uid = value.uid;

        let tileset_definition_asset_path =
            project_asset_path.to_tileset_definition_asset_path(&identifier)?;

        let cell_size = (value.c_wid, value.c_hei).into();
        let custom_data = value
            .custom_data
            .iter()
            .map(TileCustomMetadata::new)
            .collect();
        let enum_tags = value.enum_tags.iter().map(EnumTagValue::new).collect();
        let padding = value.padding;
        let size = (value.px_wid, value.px_hei).into();
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
        };

        let handle = load_context.add_labeled_asset(
            tileset_definition_asset_path.to_asset_label(),
            tileset_definition,
        );

        Ok((uid, handle))
    }
}

impl LdtkAssetWithTags for TilesetDefinition {
    fn get_tags(&self) -> &[String] {
        &self.tags
    }

    fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|inner_tag| inner_tag == tag)
    }
}
