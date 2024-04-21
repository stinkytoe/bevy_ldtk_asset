use bevy::prelude::*;

use crate::world::WorldAsset;

#[derive(Component, Debug, Default)]
#[cfg_attr(feature = "enable_reflect", derive(Reflect))]
pub struct WorldBundleLoadSettings;

#[derive(Bundle, Debug, Default)]
pub struct WorldBundle {
    pub world: Handle<WorldAsset>,
    pub settings: WorldBundleLoadSettings,
}
