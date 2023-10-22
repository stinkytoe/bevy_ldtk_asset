use crate::assets::ldtk_project::LdtkProject;
use bevy::prelude::*;

#[derive(Component, Debug, Default, Reflect)]
pub struct LdtkRoot {
    pub project: Handle<LdtkProject>,
    // pub render_set: Vec<MaterialMesh2dBundle<ColorMaterial>>,
}

// #[derive(Component, Debug, Default, Reflect)]
// pub enum LevelSet {
//     #[default]
//     All,
//     Only(Vec<String>),
// }
//
// #[derive(Component, Debug, Default, Reflect)]
// pub enum WorldSet {
//     #[default]
//     All,
//     Only(Vec<String>),
// }
//
// #[derive(Component, Debug, Default, Reflect)]
// pub struct AssetsLoadedTag;

#[derive(Component, Debug, Default, Reflect)]
pub struct Level {
    identifier: String,
    iid: String,
}
