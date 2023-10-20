use crate::assets::ldtk_project::LdtkProject;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct LdtkRoot {
    pub project: Handle<LdtkProject>,
    // pub render_set: Vec<MaterialMesh2dBundle<ColorMaterial>>,
}

#[derive(Component, Debug, Default)]
pub enum LevelSet {
    #[default]
    All,
    Only(Vec<String>),
}

#[derive(Component, Debug, Default)]
pub enum WorldSet {
    #[default]
    All,
    Only(Vec<String>),
}

#[derive(Component, Debug, Default)]
pub struct RenderTag;
