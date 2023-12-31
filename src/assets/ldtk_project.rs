use crate::ldtk_json;
use bevy::prelude::*;

/// The asset which represents an LDtk project.
#[derive(Asset, Debug, TypePath)]
pub struct LdtkProject {
    /// The rust representation of the LDtk project JSON definition [ldtk_json::LdtkJson]
    pub value: ldtk_json::LdtkJson,
    // pub levels: Vec<Handle<LdtkLevel>>,
}

impl LdtkProject {
    pub(crate) fn get_tileset_definition(&self, uid: i64) -> Option<&ldtk_json::TilesetDefinition> {
        self.value
            .defs
            .tilesets
            .iter()
            .find(|tileset_definition| tileset_definition.uid == uid)
    }

    pub(crate) fn get_entity_definition(&self, uid: i64) -> Option<&ldtk_json::EntityDefinition> {
        self.value
            .defs
            .entities
            .iter()
            .find(|entity_definition| entity_definition.uid == uid)
    }
}
