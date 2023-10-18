use crate::assets::ldtk_project::LdtkProject;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct LdtkRoot {
    pub(crate) _project: Handle<LdtkProject>,
}

#[derive(Bundle, Default)]
pub(crate) struct LdtkBundle {
    pub(crate) root: LdtkRoot,
    pub(crate) spatial_bundle: SpatialBundle,
}
