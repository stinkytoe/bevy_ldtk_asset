use crate::assets::ldtk_level::LdtkLevel;
use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct LdtkLevelBundle {
    pub level: Handle<LdtkLevel>,
    pub spatial_bundle: SpatialBundle,
}
