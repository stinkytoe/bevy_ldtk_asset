use super::{int_grid_value::IntGridValue, level_asset::LevelAsset};
use crate::ldtk_json;
use bevy::{asset::LoadState, prelude::*, utils::HashMap};

/// The asset which represents an LDtk project.
#[derive(Asset, Debug, TypePath)]
pub struct ProjectAsset {
    /// The rust representation of the LDtk project JSON definition [ldtk_json::LdtkJson]
    pub(crate) value: ldtk_json::LdtkJson,
    // pub levels: Vec<Handle<LdtkLevel>>,
    #[dependency]
    pub(crate) levels: Vec<Handle<LevelAsset>>,
}

impl ProjectAsset {
    pub(crate) fn is_loaded(&self, asset_server: &AssetServer) -> bool {
        self.levels.iter().all(|level_handle| {
            matches!(
                asset_server.get_load_state(level_handle),
                Some(LoadState::Loaded)
            )
        })
    }

    /// The rust representation of the LDtk entity JSON definition [ldtk_json::EntityDefinition]
    pub fn get_entity_definition_by_uid(&self, uid: i64) -> Option<&ldtk_json::EntityDefinition> {
        self.value
            .defs
            .entities
            .iter()
            .find(|entity_definition| entity_definition.uid == uid)
    }

    /// The rust representation of the LDtk enum JSON definition [ldtk_json::LayerDefinition]
    pub fn get_enum_definition_by_uid(&self, uid: i64) -> Option<&ldtk_json::EnumDefinition> {
        self.value
            .defs
            .enums
            .iter()
            .find(|enum_definition| enum_definition.uid == uid)
    }

    /// The rust representation of the LDtk layer JSON definition [ldtk_json::LayerDefinition]
    pub fn get_layer_definition_by_uid(&self, uid: i64) -> Option<&ldtk_json::LayerDefinition> {
        self.value
            .defs
            .layers
            .iter()
            .find(|layer_definition| layer_definition.uid == uid)
    }

    /// The rust representation of the LDtk tileset JSON definition [ldtk_json::TilesetDefinition]
    pub fn get_tileset_definition_by_uid(&self, uid: i64) -> Option<&ldtk_json::TilesetDefinition> {
        self.value
            .defs
            .tilesets
            .iter()
            .find(|tileset_definition| tileset_definition.uid == uid)
    }

    // Returns the int grid value at the given world coordinate, or None if there is either no
    // int grid value, or there is no level at that coordinate
    // pub fn get_int_grid_value_at_level_coordinate(&self, coord: Vec2) -> Option<IntGridValue> {
    //     let ret = self
    //         .value
    //         .le
    //     todo!()
    // }
}
