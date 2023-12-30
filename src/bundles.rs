use crate::{assets::ldtk_level::LdtkLevel, components::LdtkLevelComponent};
use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct LdtkLevelBundle {
    pub level: Handle<LdtkLevel>,
    pub level_component: LdtkLevelComponent,
    pub spatial_bundle: SpatialBundle,
}
