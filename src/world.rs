use bevy::prelude::*;

use crate::ldtk;

#[derive(Bundle, Debug, Default)]
pub struct WorldBundle {
    pub world: Handle<WorldAsset>,
}

#[derive(Asset, Debug, TypePath)]
pub struct WorldAsset {
    pub(crate) project_handle: Handle<crate::project::ProjectAsset>,
    pub(crate) iid: String,
}
