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
    /// A handle holding the reference to the LdtkLevel [LdtkLevel]
    pub level: Handle<LevelAsset>,
    #[doc(hidden)]
    pub level_component: LevelComponent,
    #[doc(hidden)]
    pub spatial_bundle: SpatialBundle,
}

#[derive(Bundle, Default)]
pub struct LdtkProjectBundle {
    pub project: Handle<ProjectAsset>,
    #[doc(hidden)]
    pub project_component: ProjectComponent,
    #[doc(hidden)]
    pub spatial_bundle: SpatialBundle,
}
