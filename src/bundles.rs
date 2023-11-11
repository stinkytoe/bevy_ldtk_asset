use crate::assets::ldtk_level::LdtkLevel;
use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct LdtkLevelBundle {
    // ldtk_bevy_loader
    pub level: Handle<LdtkLevel>,
    // bevy
    pub spatial_bundle: SpatialBundle,
}

// #[derive(Bundle, Default)]
// pub(crate) struct WorldBundle {
//     pub(crate) world: World,
//     pub(crate) spatial_bundle: SpatialBundle,
// }
