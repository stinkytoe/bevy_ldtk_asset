use bevy::prelude::*;

use crate::ldtk_json;

#[derive(Component, Debug, Default, Reflect)]
pub struct LdtkLevelComponent;

#[derive(Component)]
pub struct LdtkEntityComponent {
    pub value: ldtk_json::EntityInstance,
}
