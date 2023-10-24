use crate::components::LdtkRoot;
use crate::components::World;
use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct LdtkBundle {
    // ldtk_bevy_loader
    pub root: LdtkRoot,
    // bevy
    pub spatial_bundle: SpatialBundle,
}

#[derive(Bundle, Default)]
pub(crate) struct WorldBundle {
    pub(crate) world: World,
    pub(crate) spatial_bundle: SpatialBundle,
}
