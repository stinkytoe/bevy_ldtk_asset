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
    pub(crate) _value: ldtk_json::LayerInstance,
}

/// A component attached to an entity when created
#[derive(Component)]
pub struct LdtkEntityComponent {
    pub(crate) value: ldtk_json::EntityInstance,
    /// The directory where the ldtk file resides. Use this with `.join(...)`
    /// for the path of an asset referenced in the LDtk JSON, to get it's path
    /// relative to the Bevy assets folder.
    pub ldtk_project_directory: PathBuf,
}

impl LdtkEntityComponent {
    /// The rust representation of the LDtk entity instance JSON definition
    /// [ldtk_json::EntityInstance]
    // pub(crate) fn value(&self) -> &ldtk_json::EntityInstance {
    //     &self.value
    // }

    /// Checks a given entity component for the presense of the given tag.
    pub fn has_tag(&self, tag: &str) -> bool {
        self.value.tags.iter().any(|inner_tag| inner_tag == tag)
    }

    /// The size of the entity instance, as defined in the LDtk project.
    pub fn size(&self) -> Vec2 {
        Vec2::new(self.value.width as f32, self.value.height as f32)
    }

    /// Return an iterator over all field instances
    pub fn field_instances(&self) -> impl Iterator<Item = &ldtk_json::FieldInstance> {
        self.value.field_instances.iter()
    }
}
