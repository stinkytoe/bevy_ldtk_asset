//! The definition for LDtk enum values.
//!
//! Used by entities to provide special values, such as items or enemy types.
//!
//! See [Enumerations](https://ldtk.io/docs/general/editor-components/enumerations-enums/)
//! in the LDtk documentation for a description.
use std::path::{Path, PathBuf};

use bevy_asset::{Asset, Handle};
use bevy_color::Color;
use bevy_platform::collections::HashMap;
use bevy_reflect::Reflect;
use futures::future::try_join_all;

use crate::result::LdtkResult;
// use crate::asset_labels::ProjectAssetPath;
use crate::color::bevy_color_from_ldtk_int;
use crate::ldtk;
use crate::ldtk_asset_trait::LdtkAssetWithTags;
use crate::ldtk_import_error;
use crate::ldtk_path::ldtk_path_to_bevy_path;
// use crate::project_loader::ProjectContext;
use crate::tileset_definition::TilesetDefinition;
use crate::tileset_rectangle::TilesetRectangle;
use crate::uid::UidMap;

/// Data associated with a specific enum.
#[allow(missing_docs)]
#[derive(Debug, Reflect)]
pub struct EnumValueDefinition {
    pub color: Color,
    pub id: String,
    pub tile: Option<TilesetRectangle>,
}

impl EnumValueDefinition {
    pub(crate) async fn new(
        value: ldtk::EnumValueDefinition,
        tileset_definitions: &UidMap<Handle<TilesetDefinition>>,
    ) -> LdtkResult<Self> {
        let color = bevy_color_from_ldtk_int(value.color);

        let id = value.id;

        let tile = value
            .tile_rect
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
    pub(crate) async fn new(
        enum_definition_json: ldtk::EnumDefinition,
        tileset_definitions: &UidMap<Handle<TilesetDefinition>>,
        project_directory: &Path,
    ) -> LdtkResult<Self> {
        let identifier = enum_definition_json.identifier.clone();

        let external_rel_path = enum_definition_json
            .external_rel_path
            .as_ref()
            .map(|path| ldtk_path_to_bevy_path(project_directory, path));

        let icon_tileset_definition = enum_definition_json
            .icon_tileset_uid
            .map(|uid| {
                tileset_definitions
                    .get(&uid)
                    .ok_or(ldtk_import_error!("bad tileset definition uid! {}", uid))
            })
            .transpose()?
            .cloned();

        let tags = enum_definition_json.tags;

        let values = enum_definition_json.values.into_iter().map(|value| async {
            let id = value.id.clone();
            let enum_value_definition =
                EnumValueDefinition::new(value, tileset_definitions).await?;
            LdtkResult::Ok((id, enum_value_definition))
        });

        let values = try_join_all(values).await?.into_iter().collect();

        Ok(Self {
            identifier,
            external_rel_path,
            icon_tileset_definition,
            tags,
            values,
        })
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
