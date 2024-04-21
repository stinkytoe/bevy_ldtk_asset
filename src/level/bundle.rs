use bevy::prelude::*;

use crate::level::LevelAsset;

#[derive(Debug, Default)]
#[cfg_attr(feature = "enable_reflect", derive(Reflect))]
pub enum LoadLayers {
    None,
    ByIdentifiers(Vec<String>),
    ByIids(Vec<String>),
    #[default]
    All,
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "enable_reflect", derive(Reflect))]
pub enum LoadEntities {
    None,
    ByIdentifiers(Vec<String>),
    ByIids(Vec<String>),
    ByTags(Vec<String>),
    #[default]
    All,
}

#[derive(Component, Debug, Default)]
#[cfg_attr(feature = "enable_reflect", derive(Reflect))]
pub struct LevelBundleLoadSettings {
    load_layers: LoadLayers,
    load_entities: LoadEntities,
}

#[derive(Bundle, Debug, Default)]
pub struct LevelBundle {
    pub level: Handle<LevelAsset>,
    pub load_settings: LevelBundleLoadSettings,
}
