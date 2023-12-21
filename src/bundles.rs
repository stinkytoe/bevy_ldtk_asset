use crate::{assets::ldtk_level::LdtkLevel, components::LevelComponent};
use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct LdtkLevelBundle {
    pub level: Handle<LdtkLevel>,
    pub level_component: LevelComponent,
    pub spatial_bundle: SpatialBundle,
}
