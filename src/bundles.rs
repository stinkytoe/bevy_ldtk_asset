use crate::{assets::ldtk_level::LdtkLevel, components::LdtkLevelComponent};
use bevy::prelude::*;

/// The bundle meant for spawning an LDtk level into the world.
/// TODO elaborate with a good sample example, or link one of the
/// example programs.
#[derive(Bundle, Default)]
pub struct LdtkLevelBundle {
    /// A handle holding the reference to the LdtkLevel [LdtkLevel]
    pub level: Handle<LdtkLevel>,
    #[doc(hidden)]
    pub level_component: LdtkLevelComponent,
    #[doc(hidden)]
    pub spatial_bundle: SpatialBundle,
}
