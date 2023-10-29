use crate::assets::ldtk_project::LdtkProject;
use bevy::prelude::*;

#[derive(Component, Debug, Default, Reflect)]
pub struct LdtkRoot {
    pub project: Handle<LdtkProject>,
    // pub render_set: Vec<MaterialMesh2dBundle<ColorMaterial>>,
}

#[derive(Component, Debug, Default, Reflect)]
pub struct World {
    pub iid: String,
}

#[derive(Component, Debug, Default, Reflect)]
pub struct Level {
    pub iid: String,
}