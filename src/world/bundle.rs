use bevy::prelude::*;

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
    load_levels: LoadLevels,
}

#[derive(Bundle, Debug, Default)]
pub struct WorldBundle {
    pub world: Handle<WorldAsset>,
    pub settings: WorldBundleLoadSettings,
}
