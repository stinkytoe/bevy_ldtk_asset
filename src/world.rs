use bevy::prelude::*;

use crate::project::ProjectAsset;

#[derive(Bundle, Debug, Default)]
pub struct WorldBundle {
    pub world: Handle<WorldAsset>,
}

#[derive(Asset, Debug, TypePath)]
pub struct WorldAsset {
    pub(crate) _project_handle: Handle<ProjectAsset>,
    pub(crate) _iid: String,
}
