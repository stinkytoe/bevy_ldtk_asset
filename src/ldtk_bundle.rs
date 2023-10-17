use crate::assets::ldtk_project::LdtkProject;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct LdtkRoot {
    pub project: Handle<LdtkProject>,
}

#[derive(Component, Default)]
pub struct LdtkRootLevelLoadStub;

#[derive(Bundle, Default)]
pub struct LdtkBundle {
    // pub root: Handle<LdtkRoot>,
    pub root: LdtkRoot,
    pub spatial_bundle: SpatialBundle,
    pub level_load_stub: LdtkRootLevelLoadStub,
}
