//! The definition for LDtk enum values.
//!
//! Used by entities to provide special values, such as items or enemy types.
//!
//! See [Enumerations](https://ldtk.io/docs/general/editor-components/enumerations-enums/)
//! in the LDtk documentation for a description.
use std::path::PathBuf;

use bevy_asset::{Asset, Handle, LoadContext};
use bevy_color::Color;
use bevy_platform_support::collections::HashMap;
use bevy_reflect::Reflect;

use crate::asset_labels::ProjectAssetPath;
use crate::color::bevy_color_from_ldtk_int;
use crate::ldtk;
use crate::ldtk_asset_trait::LdtkAssetWithTags;
use crate::ldtk_import_error;
use crate::ldtk_path::ldtk_path_to_bevy_path;
use crate::project_loader::ProjectContext;
use crate::tileset_definition::TilesetDefinition;
use crate::tileset_rectangle::TilesetRectangle;
use crate::uid::UidMap;
use crate::Result;

/// Data associated with a specific enum.
#[allow(missing_docs)]
#[derive(Debug, Reflect)]
pub struct EnumValueDefinition {
    pub color: Color,
    pub id: String,
    pub tile: Option<TilesetRectangle>,
}

impl EnumValueDefinition {
    pub(crate) fn new(
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

/// An enum definition groups a vector of [EnumValueDefinition]s into a logical group.
#[allow(missing_docs)]
#[derive(Asset, Debug, Reflect)]
pub struct EnumDefinition {
    // Enum definitions are a little different in the way they are indexed. They do have internal
    // Uids, however, a field instance of type Enum or Array<Enum> doesn't expose this directly. The
    // data is in the JSON, but it's not exposed via the schema. So, we index these by their
    // identifier as opposed to their Uid. Any definition *could* be indexed the same way, but I chose
    // to use the uid when available via the schema.
    pub identifier: String,
    pub external_rel_path: Option<PathBuf>,
    icon_tileset_definition: Option<Handle<TilesetDefinition>>,
    pub tags: Vec<String>,
    pub values: HashMap<String, EnumValueDefinition>,
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
            .map(|value| {
                Ok((
                    value.id.clone(),
                    EnumValueDefinition::new(value, tileset_definitions)?,
                ))
            })
            .collect::<Result<_>>()?;

        let path = project_asset_path.to_enum_definition_asset_path(&identifier)?;

        let asset = Self {
            identifier: identifier.clone(),
            external_rel_path,
            icon_tileset_definition,
            tags,
            values,
        };

        let handle = load_context.add_labeled_asset(path.to_asset_label(), asset)?;

        Ok((identifier, handle))
    }
}

impl EnumDefinition {
    /// Returns true iff this enum definition has the given tag in its tags field. This is filled
    /// out in the editor, by the level designer.
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|inner_tag| inner_tag == tag)
    }
}

impl LdtkAssetWithTags for EnumDefinition {
    fn get_tags(&self) -> &[String] {
        &self.tags
    }

    fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|inner_tag| inner_tag == tag)
    }
}
