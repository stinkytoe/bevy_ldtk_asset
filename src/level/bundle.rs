use bevy::prelude::*;

use crate::level::LevelAsset;

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "enable_reflect", derive(Reflect))]
pub enum LoadLayers {
    None,
    ByIdentifiers(Vec<String>),
    ByIids(Vec<String>),
    #[default]
    All,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "enable_reflect", derive(Reflect))]
pub enum LoadEntities {
    None,
    ByIdentifiers(Vec<String>),
    ByIids(Vec<String>),
    ByTags(Vec<String>),
    #[default]
    All,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "enable_reflect", derive(Reflect))]
pub enum LoadEntityMode {
    ComponentOnly,
    #[default]
    Sprite,
}

#[derive(Clone, Component, Debug, Default)]
#[cfg_attr(feature = "enable_reflect", derive(Reflect))]
pub struct LevelBundleLoadSettings {
    pub load_layers: LoadLayers,
    pub load_entities: LoadEntities,
    pub load_entity_mode: LoadEntityMode,
}

#[derive(Bundle, Debug, Default)]
pub struct LevelBundle {
    pub level: Handle<LevelAsset>,
    pub load_settings: LevelBundleLoadSettings,
}
