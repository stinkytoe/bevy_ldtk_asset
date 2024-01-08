use crate::ldtk::{level_asset::LevelAsset, level_component::LevelComponent};
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
