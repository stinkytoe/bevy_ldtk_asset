use std::collections::HashMap;

use bevy::prelude::*;

use crate::assets::ldtk_project::LdtkProject;

#[derive(Component, Default)]
pub struct LdtkRoot {
    pub root: Handle<LdtkProject>,
}

#[derive(Component, Default)]
pub enum LdtkBackgroundLoader {
    #[default]
    Uninitialized,
    Initialized {
        backgrounds: Vec<String>,
    },
}

#[derive(Component, Default)]
pub struct LdtkBackgrounds {
    pub backgrounds: HashMap<String, Handle<Image>>,
}

#[derive(Bundle, Default)]
pub struct LdtkBundle {
    // pub root: Handle<LdtkRoot>,
    pub root: LdtkRoot,
    pub spatial_bundle: SpatialBundle,
    pub background_loader: LdtkBackgroundLoader,
    pub backgrounds: LdtkBackgrounds,
}
