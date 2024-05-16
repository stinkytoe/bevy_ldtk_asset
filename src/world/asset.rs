use bevy::prelude::*;

use crate::project::ProjectAsset;

#[derive(Asset, Debug, Reflect)]
pub struct WorldAsset {
    #[reflect(ignore)]
    pub(crate) project: Handle<ProjectAsset>,
    pub(crate) iid: String,
}
