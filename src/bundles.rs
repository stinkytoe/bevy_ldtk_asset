use crate::components::LdtkRoot;
use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct LdtkBundle {
    // ldtk_bevy_loader
    pub root: LdtkRoot,
    // bevy
    pub spatial_bundle: SpatialBundle,
}
