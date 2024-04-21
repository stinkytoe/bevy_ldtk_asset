use bevy::prelude::*;

use crate::level::LevelAsset;

// This component is immediately removed,
// and the appropriate assets/components/child entities/etc are added
#[derive(Component, Debug)]
#[cfg_attr(feature = "enable_reflect", derive(Reflect))]
pub struct LevelBundleLoadSettings {
    bg_color: bool,
    bg_image: bool,
    layers: bool,
    entities: bool,
}

impl LevelBundleLoadSettings {
    pub fn bg_color(&self) -> bool {
        self.bg_color
    }

    pub fn bg_image(&self) -> bool {
        self.bg_image
    }

    pub fn layers(&self) -> bool {
        self.layers
    }

    pub fn entities(&self) -> bool {
        self.entities
    }
}

impl Default for LevelBundleLoadSettings {
    fn default() -> Self {
        Self {
            bg_color: true,
            bg_image: true,
            layers: true,
            entities: true,
        }
    }
}

#[derive(Bundle, Debug, Default)]
pub struct LevelBundle {
    pub level: Handle<LevelAsset>,
    pub load_settings: LevelBundleLoadSettings,
}
