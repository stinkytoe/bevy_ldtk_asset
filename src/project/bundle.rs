use bevy::prelude::*;

use crate::project::ProjectAsset;
use crate::project::WorldsToLoad;

#[derive(Bundle, Debug, Default)]
pub struct ProjectBundle {
    pub project: Handle<ProjectAsset>,
    pub worlds_to_load: WorldsToLoad,
    pub spatial: SpatialBundle,
}
