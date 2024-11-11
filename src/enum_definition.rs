use std::path::PathBuf;

use bevy_asset::{Asset, Handle, LoadContext};
use bevy_color::Color;
use bevy_reflect::Reflect;

use crate::asset_labels::ProjectAssetPath;
use crate::color::bevy_color_from_ldtk_int;
use crate::ldtk_path::ldtk_path_to_bevy_path;
use crate::project_loader::ProjectContext;
use crate::tileset_definition::TilesetDefinition;
use crate::tileset_rectangle::TilesetRectangle;
use crate::uid::UidMap;
use crate::Result;
use crate::{ldtk, ldtk_import_error};

#[derive(Debug, Reflect)]
pub struct EnumValueDefinition {
    pub color: Color,
    pub id: String,
    pub tile: Option<TilesetRectangle>,
}

impl EnumValueDefinition {
    pub fn new(
        value: &ldtk::EnumValueDefinition,
        tileset_definitions: &UidMap<Handle<TilesetDefinition>>,
    ) -> Result<Self> {
        let color = bevy_color_from_ldtk_int(value.color);
        let id = value.id.clone();
        let tile = value
            .tile_rect
            .as_ref()
            .map(|value| TilesetRectangle::new(value, tileset_definitions))
            .transpose()?;

        Ok(Self { color, id, tile })
    }
}

#[derive(Asset, Debug, Reflect)]
pub struct EnumDefinition {
    pub identifier: String,
    pub external_rel_path: Option<PathBuf>,
    //pub icon_tileset_uid: Option<i64>,
    icon_tileset_definition: Option<Handle<TilesetDefinition>>,
    pub tags: Vec<String>,
    pub values: Vec<EnumValueDefinition>,
}

impl EnumDefinition {
    pub(crate) fn create_handle_pair(
        value: &ldtk::EnumDefinition,
        project_asset_path: &ProjectAssetPath,
        load_context: &mut LoadContext,
        project_context: &ProjectContext,
        tileset_definitions: &UidMap<Handle<TilesetDefinition>>,
    ) -> Result<(i64, Handle<Self>)> {
        let uid = value.uid;
        let identifier = value.identifier.clone();
        let external_rel_path = value
            .external_rel_path
            .as_ref()
            .map(|path| ldtk_path_to_bevy_path(project_context.project_directory, path));
        let icon_tileset_definition = value
            .icon_tileset_uid
            .as_ref()
            .map(|uid| {
                tileset_definitions
                    .get(uid)
                    .ok_or(ldtk_import_error!("bad tileset definition uid! {}", uid))
            })
            .transpose()?
            .cloned();
        let tags = value.tags.clone();
        let values = value
            .values
            .iter()
            .map(|value| EnumValueDefinition::new(value, tileset_definitions))
            .collect::<Result<Vec<_>>>()?;

        let path = project_asset_path.to_enum_definition_asset_path(&identifier)?;

        let asset = Self {
            identifier,
            external_rel_path,
            icon_tileset_definition,
            tags,
            values,
        }
        .into();

        let handle = load_context.add_loaded_labeled_asset(path.to_asset_label(), asset);

        Ok((uid, handle))
    }
}
