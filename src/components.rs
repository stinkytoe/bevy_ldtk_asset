use crate::assets::ldtk_project::LdtkProject;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct LdtkRoot {
    pub project: Handle<LdtkProject>,
}

#[derive(Component, Debug, Default)]
pub enum IidSet {
    #[default]
    All,
    _Only(Vec<String>),
}
