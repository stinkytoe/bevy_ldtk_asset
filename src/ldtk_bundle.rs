use crate::assets::ldtk_project::LdtkProject;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct LdtkRoot {
    pub project: Handle<LdtkProject>,
}

#[derive(Bundle, Default)]
pub struct LdtkBundle {
    pub root: LdtkRoot,
    pub spatial_bundle: SpatialBundle,
}
