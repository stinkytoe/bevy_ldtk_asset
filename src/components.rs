use crate::assets::ldtk_project::LdtkProject;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct LdtkRoot {
    pub project: Handle<LdtkProject>,
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
