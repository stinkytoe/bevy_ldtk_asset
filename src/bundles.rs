use crate::components::{IidSet, LdtkRoot};
use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct LdtkBundle {
    // ldtk_bevy_loader
    pub root: LdtkRoot,
    pub world_set: IidSet,
    pub level_set: IidSet,
    // bevy
    pub spatial_bundle: SpatialBundle,
}
