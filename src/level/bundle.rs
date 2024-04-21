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
pub enum LoadLayerMode {
    ComponentOnly,
    #[default]
    Mesh,
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

#[derive(Clone, Component, Debug)]
#[cfg_attr(feature = "enable_reflect", derive(Reflect))]
pub struct LevelBundleLoadSettings {
    pub load_bg_color: bool,
    pub load_bg_image: bool,
    pub load_layers: LoadLayers,
    pub load_layer_mode: LoadLayerMode,
    pub load_entities: LoadEntities,
    pub load_entity_mode: LoadEntityMode,
}

impl Default for LevelBundleLoadSettings {
    fn default() -> Self {
        Self {
            load_bg_color: true,
            load_bg_image: true,
            load_layers: Default::default(),
            load_layer_mode: Default::default(),
            load_entities: Default::default(),
            load_entity_mode: Default::default(),
        }
    }
}

#[derive(Bundle, Debug, Default)]
pub struct LevelBundle {
    pub level: Handle<LevelAsset>,
    pub load_settings: LevelBundleLoadSettings,
}
