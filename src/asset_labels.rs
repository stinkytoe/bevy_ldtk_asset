//! Asset labels for the sub-asset types.
//!
//! Each world is stored in its own sub asset of type [crate::world::World], and labeled with the prefix of
//! `worlds:WorldIdentifier`.
//!
//! Levels are of type [crate::level::Level], and labeled with their respective world with the label
//! `worlds:WorldIdentifier/LevelIdentifier`.
//!
//! Layers are of type [crate::layer::Layer], and labeled with their respective level and world as
//! `worlds:WorldIdentifier/LevelIdentifier/LayerIdentifier`. Layer names are not unique globally,
//! but are unique when paired with their containing level.
//!
//! Entities are of type [crate::entity::Entity]. They differ from the other types in that their identifiers are
//! not unique. A user can make multiple copies of any given entity, barring restrictions in LDtk
//! itself. (see [Entities](https://ldtk.io/json/#ldtk-EntityInstanceJson) section of the LDtk
//! documentation). To maintain uniqueness, entities in this library will label themselves with a
//! concatenation of the identifier and the string representation of their [Iid] like so:
//! `Entity@deadbeef-1234-fee1-5678-ba5eba118ba7`. This will be appended to the layer instance they
//! belong to in the editor:
//! `worlds:WorldIdentifier/LevelIdentifier/LayerIdentifier/Entity@deadbeef-1234-fee1-5678-ba5eba118ba7`.
//!
//! [crate::tileset_definition::TilesetDefinition]s are labeled as: `tileset_definitions:Identifier`.
//!
//! [crate::entity_definition::EntityDefinition]s are labeled as: `entity_definitions:Identifier`.
//!
//! [crate::layer_definition::LayerDefinition]s are labeled as: `layer_definitions:Identifier`.
//!
//! [crate::enum_definition::EnumDefinition]s are labeled as: `enum_definitions:Identifier`.

#![allow(missing_docs)]
use bevy_reflect::Reflect;

use crate::iid::Iid;
use crate::{ldtk_import_error, Result};

#[derive(Debug, PartialEq, Eq, Reflect)]
pub struct ProjectAssetPath {
    path: String,
}

impl ProjectAssetPath {
    pub fn new(path: &str) -> Result<Self> {
        validate_path(path)
            .then(|| Self {
                path: path.to_string(),
            })
            .ok_or(ldtk_import_error!(
                "Could not construct asset path from {:?}!",
                path
            ))
    }

    pub fn to_entity_definition_asset_path(
        &self,
        entity_definition_identifier: &str,
    ) -> Result<EntityDefinitionAssetPath> {
        validate_label(entity_definition_identifier)
            .then(|| EntityDefinitionAssetPath {
                path: self.path.clone(),
                entity_definition_identifier: entity_definition_identifier.to_string(),
            })
            .ok_or(ldtk_import_error!(
                "Could not construct entity definition path from {:?}!",
                entity_definition_identifier
            ))
    }

    pub fn to_layer_definition_asset_path(
        &self,
        layer_definition_identifier: &str,
    ) -> Result<LayerDefinitionAssetPath> {
        validate_label(layer_definition_identifier)
            .then(|| LayerDefinitionAssetPath {
                path: self.path.clone(),
                layer_definition_identifier: layer_definition_identifier.to_string(),
            })
            .ok_or(ldtk_import_error!(
                "Could not construct layer definition path from {:?}!",
                layer_definition_identifier
            ))
    }

    pub fn to_tileset_definition_asset_path(
        &self,
        tileset_definition_identifier: &str,
    ) -> Result<TilesetDefinitionAssetPath> {
        validate_label(tileset_definition_identifier)
            .then(|| TilesetDefinitionAssetPath {
                path: self.path.clone(),
                tileset_definition_identifier: tileset_definition_identifier.to_string(),
            })
            .ok_or(ldtk_import_error!(
                "Could not construct tileset definition asset path from token {tileset_definition_identifier:?}!"
            ))
    }

    pub fn to_enum_definition_asset_path(
        &self,
        enum_definition_identifier: &str,
    ) -> Result<EnumDefinitionAssetPath> {
        validate_label(enum_definition_identifier).then(|| EnumDefinitionAssetPath {
                path: self.path.clone(),
                enum_definition_identifier: enum_definition_identifier.to_string(),
            })
            .ok_or(ldtk_import_error!(
                "Could not construct enum definition asset path from token {enum_definition_identifier:?}!"
            ))
    }

    pub fn to_world_asset_path(&self, world_identifier: &str) -> Result<WorldAssetPath> {
        validate_label(world_identifier)
            .then_some(WorldAssetPath {
                path: self.path.clone(),
                world_identifier: world_identifier.to_string(),
            })
            .ok_or(ldtk_import_error!(
                "Could not construct world asset path from world token {world_identifier:?}!"
            ))
    }
}

#[derive(Debug, Reflect)]
pub struct EntityDefinitionAssetPath {
    path: String,
    entity_definition_identifier: String,
}

impl EntityDefinitionAssetPath {
    pub fn to_asset_label(&self) -> String {
        format!("entity_definitions:{}", self.entity_definition_identifier)
    }
}

#[derive(Debug, Reflect)]
pub struct LayerDefinitionAssetPath {
    path: String,
    layer_definition_identifier: String,
}

impl LayerDefinitionAssetPath {
    pub fn to_asset_label(&self) -> String {
        format!("layer_definitions:{}", self.layer_definition_identifier)
    }
}

#[derive(Debug, Reflect)]
pub struct TilesetDefinitionAssetPath {
    path: String,
    tileset_definition_identifier: String,
}

impl TilesetDefinitionAssetPath {
    pub fn to_asset_label(&self) -> String {
        format!("tileset_definitions:{}", self.tileset_definition_identifier)
    }
}

#[derive(Debug, Reflect)]
pub struct EnumDefinitionAssetPath {
    path: String,
    enum_definition_identifier: String,
}

impl EnumDefinitionAssetPath {
    pub fn to_asset_label(&self) -> String {
        format!("enum_definitions:{}", self.enum_definition_identifier)
    }
}

#[derive(Debug, Reflect, PartialEq, Eq)]
pub struct WorldAssetPath {
    path: String,
    world_identifier: String,
}

impl WorldAssetPath {
    pub fn to_level_asset_path(&self, level_identifier: &str) -> Result<LevelAssetPath> {
        validate_label(level_identifier)
            .then(|| LevelAssetPath {
                path: self.path.clone(),
                world_identifier: self.world_identifier.clone(),
                level_identifier: level_identifier.to_string(),
            })
            .ok_or(ldtk_import_error!(
                "Could not construct level asset path from level token {level_identifier:?}!"
            ))
    }

    pub fn to_asset_path(&self) -> String {
        format!("{}#{}", self.path, self.to_asset_label())
    }

    pub fn to_asset_label(&self) -> String {
        format!("worlds:{}", self.world_identifier)
    }
}

#[derive(Debug, Reflect)]
pub struct LevelAssetPath {
    path: String,
    world_identifier: String,
    level_identifier: String,
}

impl LevelAssetPath {
    pub fn to_layer_asset_path(&self, layer_identifier: &str) -> Result<LayerAssetPath> {
        validate_label(layer_identifier)
            .then(|| LayerAssetPath {
                path: self.path.clone(),
                world_identifier: self.world_identifier.clone(),
                level_identifier: self.level_identifier.clone(),
                layer_identifier: layer_identifier.to_string(),
            })
            .ok_or(ldtk_import_error!(
                "Could not construct layer asset path from layer token {layer_identifier:?}!"
            ))
    }

    pub fn to_asset_label(&self) -> String {
        format!("worlds:{}/{}", self.world_identifier, self.level_identifier)
    }
}

#[derive(Debug, Reflect)]
pub struct LayerAssetPath {
    path: String,
    world_identifier: String,
    level_identifier: String,
    layer_identifier: String,
}

impl LayerAssetPath {
    pub fn to_entity_asset_path(
        &self,
        entity_identifier: &str,
        entity_iid: Iid,
    ) -> Result<EntityAssetPath> {
        validate_label(entity_identifier)
            .then(|| EntityAssetPath {
                path: self.path.clone(),
                world_identifier: self.world_identifier.clone(),
                level_identifier: self.level_identifier.clone(),
                layer_identifier: self.layer_identifier.clone(),
                entity_identifier: entity_identifier.to_string(),
                entity_iid,
            })
            .ok_or(ldtk_import_error!(
                "Could not construct entity asset path from entity token {entity_identifier:?}!"
            ))
    }
    pub fn to_asset_label(&self) -> String {
        format!(
            "worlds:{}/{}/{}",
            self.world_identifier, self.level_identifier, self.layer_identifier
        )
    }
}

#[derive(Debug, Reflect)]
pub struct EntityAssetPath {
    path: String,
    world_identifier: String,
    level_identifier: String,
    layer_identifier: String,
    entity_identifier: String,
    entity_iid: Iid,
}

impl EntityAssetPath {
    pub fn to_asset_label(&self) -> String {
        format!(
            "worlds:{}/{}/{}/{}@{}",
            self.world_identifier,
            self.level_identifier,
            self.layer_identifier,
            self.entity_identifier,
            self.entity_iid
        )
    }
}

// FIXME: do something here!!!
fn validate_path(_value: &str) -> bool {
    true
}

fn validate_label(value: &str) -> bool {
    const VALID_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_";

    value.chars().all(|ch| VALID_CHARS.contains(ch))
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod test {
    use super::ProjectAssetPath;
    use crate::iid::Iid;

    #[test]
    fn test() {
        let path = "path/to/test.ldtk".to_string();
        let project_asset_path = ProjectAssetPath::new(&path).unwrap();

        let world_identifier = "Lalaland_123".to_string();

        let world_asset_path = project_asset_path
            .to_world_asset_path(&world_identifier)
            .unwrap();
        assert_eq!(
            world_asset_path.to_asset_path(),
            "path/to/test.ldtk#worlds:Lalaland_123"
        );
        assert_eq!(world_asset_path.to_asset_label(), "worlds:Lalaland_123");

        let level_identifier = "FirstLevel".to_string();
        let level_asset_path = world_asset_path
            .to_level_asset_path(&level_identifier)
            .unwrap();
        assert_eq!(
            level_asset_path.to_asset_label(),
            "worlds:Lalaland_123/FirstLevel"
        );

        let layer_identifier = "HotLava".to_string();
        let layer_asset_path = level_asset_path
            .to_layer_asset_path(&layer_identifier)
            .unwrap();
        assert_eq!(
            layer_asset_path.to_asset_label(),
            "worlds:Lalaland_123/FirstLevel/HotLava"
        );

        use std::str::FromStr;
        let entity_identifier = "Bauble_Pile".to_string();
        let entity_iid_str = "deadbeef-1234-fee1-5678-ba5eba118ba7";
        let entity_iid = Iid::from_str(entity_iid_str).unwrap();
        let entity_asset_path = layer_asset_path
            .to_entity_asset_path(&entity_identifier, entity_iid)
            .unwrap();
        assert_eq!(
            entity_asset_path.to_asset_label(),
            "worlds:Lalaland_123/FirstLevel/HotLava/Bauble_Pile@deadbeef-1234-fee1-5678-ba5eba118ba7"
        );

        let tileset_definition_identifier = "FishScales".to_string();
        let tileset_asset_path = project_asset_path
            .to_tileset_definition_asset_path(&tileset_definition_identifier)
            .unwrap();
        assert_eq!(
            tileset_asset_path.to_asset_label(),
            "tileset_definitions:FishScales"
        );

        let entity_definition_identifier = "BogBoss".to_string();
        let entity_definition_path = project_asset_path
            .to_entity_definition_asset_path(&entity_definition_identifier)
            .unwrap();
        assert_eq!(
            entity_definition_path.to_asset_label(),
            "entity_definitions:BogBoss"
        );

        let layer_definition_identifier = "NougatLayer".to_string();
        let layer_definition_path = project_asset_path
            .to_layer_definition_asset_path(&layer_definition_identifier)
            .unwrap();
        assert_eq!(
            layer_definition_path.to_asset_label(),
            "layer_definitions:NougatLayer"
        );

        let enum_definition_identifier = "CowTools".to_string();
        let enum_definition_path = project_asset_path
            .to_enum_definition_asset_path(&enum_definition_identifier)
            .unwrap();
        assert_eq!(
            enum_definition_path.to_asset_label(),
            "enum_definitions:CowTools"
        );

        assert!(project_asset_path.to_world_asset_path("garbage!").is_err());
        assert!(world_asset_path.to_level_asset_path("has spaces?").is_err());
        assert!(level_asset_path.to_layer_asset_path("--dashes--").is_err());
        assert!(layer_asset_path
            .to_entity_asset_path("--dashes--", entity_iid)
            .is_err());
        assert!(project_asset_path
            .to_tileset_definition_asset_path("Tá Unicode uamhnach!")
            .is_err());
        assert!(project_asset_path
            .to_layer_definition_asset_path("यूनिकोड कमाल")
            .is_err());
        assert!(project_asset_path
            .to_entity_definition_asset_path("Юнікод чудовий!")
            .is_err());
        assert!(project_asset_path
            .to_enum_definition_asset_path("Unicode គឺអស្ចារ្យណាស់!")
            .is_err());
    }
}
