use std::path::PathBuf;

use crate::ldtk_json;
use bevy::prelude::*;

/// A component for identifying this as an LdtkLevel in the ECS system
#[derive(Component, Debug, Default, Reflect)]
pub struct LdtkLevelComponent;

/// A component attached to a layer when created
#[derive(Component)]
pub struct LdtkLayerComponent {
    /// The rust representation of the LDtk layer instance JSON definition
    /// [ldtk_json::LayerInstance]
    pub value: ldtk_json::LayerInstance,
}

/// A component attached to an entity when created
#[derive(Component)]
pub struct LdtkEntityComponent {
    /// The rust representation of the LDtk entity instance JSON definition
    /// [ldtk_json::EntityInstance]
    pub value: ldtk_json::EntityInstance,
    /// The directory where the ldtk file resides. Use this with `.join(...)`
    /// for the path of an asset referenced in the LDtk JSON, to get it's path
    /// relative to the Bevy assets folder.
    pub ldtk_project_directory: PathBuf,
}

impl LdtkEntityComponent {
    /// Checks a given entity component for the presense of the given tag.
    pub fn has_tag(&self, tag: &str) -> bool {
        self.value.tags.iter().any(|inner_tag| inner_tag == tag)
    }
}
