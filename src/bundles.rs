use crate::ldtk::{
    level_asset::LevelAsset, level_component::LevelComponent, project_asset::ProjectAsset,
    project_component::ProjectComponent,
};
use bevy::prelude::*;

/// The bundle meant for spawning an LDtk level into the world.
/// TODO elaborate with a good sample example, or link one of the
/// example programs.
#[derive(Bundle, Default)]
pub struct LdtkLevelBundle {
    /// A handle holding the reference to the Level Asset [LevelAsset]
    pub level: Handle<LevelAsset>,
    #[doc(hidden)]
    pub level_component: LevelComponent,
    #[doc(hidden)]
    pub spatial_bundle: SpatialBundle,
}

/// The bundle user for spawning an entire LDtk project.
#[derive(Bundle, Default)]
pub struct LdtkProjectBundle {
    /// A handle holding the reference to the Project Asset [ProjectAsset]
    pub project: Handle<ProjectAsset>,
    #[doc(hidden)]
    pub project_component: ProjectComponent,
    #[doc(hidden)]
    pub spatial_bundle: SpatialBundle,
}
