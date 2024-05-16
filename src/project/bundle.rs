use bevy::prelude::*;

use crate::project::ProjectAsset;
use crate::world::WorldsToLoad;

#[derive(Bundle, Debug, Default)]
pub struct ProjectBundle {
    pub project: Handle<ProjectAsset>,
    pub worlds_to_load: WorldsToLoad,
    pub spatial: SpatialBundle,
}
