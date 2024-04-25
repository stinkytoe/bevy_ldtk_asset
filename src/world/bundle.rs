use bevy::prelude::*;

use crate::level::LevelBundleLoadSettings;
use crate::world::WorldAsset;

#[derive(Debug, Default)]
#[cfg_attr(feature = "enable_reflect", derive(Reflect))]
pub enum LoadLevels {
    None,
    ByIdentifiers(Vec<String>),
    ByIids(Vec<String>),
    #[default]
    All,
}

#[derive(Component, Debug, Default)]
#[cfg_attr(feature = "enable_reflect", derive(Reflect))]
pub struct WorldBundleLoadSettings {
    pub load_levels: LoadLevels,
    pub level_bundle_load_settings: LevelBundleLoadSettings,
}

#[derive(Bundle, Debug, Default)]
pub struct WorldBundle {
    pub world: Handle<WorldAsset>,
    pub settings: WorldBundleLoadSettings,
}
