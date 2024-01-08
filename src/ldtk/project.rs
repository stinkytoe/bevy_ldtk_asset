use crate::ldtk_json;
use bevy::prelude::*;

/// The asset which represents an LDtk project.
#[derive(Asset, Debug, TypePath)]
pub struct Project {
    /// The rust representation of the LDtk project JSON definition [ldtk_json::LdtkJson]
    pub(crate) value: ldtk_json::LdtkJson,
    // pub levels: Vec<Handle<LdtkLevel>>,
}

impl Project {
    /// The rust representation of the LDtk entity JSON definition [ldtk_json::EntityDefinition]
    pub fn get_entity_definition(&self, uid: i64) -> Option<&ldtk_json::EntityDefinition> {
        self.value
            .defs
            .entities
            .iter()
            .find(|entity_definition| entity_definition.uid == uid)
    }

    /// The rust representation of the LDtk enum JSON definition [ldtk_json::LayerDefinition]
    pub fn get_enum_definition(&self, uid: i64) -> Option<&ldtk_json::EnumDefinition> {
        self.value
            .defs
            .enums
            .iter()
            .find(|enum_definition| enum_definition.uid == uid)
    }

    /// The rust representation of the LDtk layer JSON definition [ldtk_json::LayerDefinition]
    pub fn get_layer_definition(&self, uid: i64) -> Option<&ldtk_json::LayerDefinition> {
        self.value
            .defs
            .layers
            .iter()
            .find(|layer_definition| layer_definition.uid == uid)
    }

    /// The rust representation of the LDtk tileset JSON definition [ldtk_json::TilesetDefinition]
    pub fn get_tileset_definition(&self, uid: i64) -> Option<&ldtk_json::TilesetDefinition> {
        self.value
            .defs
            .tilesets
            .iter()
            .find(|tileset_definition| tileset_definition.uid == uid)
    }
}
