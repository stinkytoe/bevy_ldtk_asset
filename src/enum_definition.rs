use std::path::PathBuf;

use bevy_asset::{Asset, Handle, LoadContext};
use bevy_color::Color;
use bevy_reflect::Reflect;

use crate::asset_labels::ProjectAssetPath;
use crate::color::bevy_color_from_ldtk_int;
use crate::ldtk;
use crate::ldtk_import_error;
use crate::ldtk_path::ldtk_path_to_bevy_path;
use crate::project_loader::ProjectContext;
use crate::tileset_definition::TilesetDefinition;
use crate::tileset_rectangle::TilesetRectangle;
use crate::uid::UidMap;
use crate::Result;

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

/// Enum definitions are a little different in the way they are indexed. They do have internal
/// Uids, however, a field instance of type Enum or Array<Enum> doesn't expose this directly. The
/// data is in the JSON, but it's not exposed via the schema. So, we index these by their
/// identifier as opposed to their Uid. Any definition *could* be indexed the same way, but I chose
/// to use the uid when available via the schema.
#[derive(Asset, Debug, Reflect)]
pub struct EnumDefinition {
    pub identifier: String,
    pub external_rel_path: Option<PathBuf>,
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
    ) -> Result<(String, Handle<Self>)> {
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
            identifier: identifier.clone(),
            external_rel_path,
            icon_tileset_definition,
            tags,
            values,
        }
        .into();

        let handle = load_context.add_loaded_labeled_asset(path.to_asset_label(), asset);

        Ok((identifier, handle))
    }
}

impl EnumDefinition {
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|inner_tag| inner_tag == tag)
    }
}
