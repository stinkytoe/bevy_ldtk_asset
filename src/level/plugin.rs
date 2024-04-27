use bevy::prelude::*;
use bevy::utils::error;

use crate::level::level_bundle_loaded;
use crate::level::new_level_bundle;
use crate::level::LevelAsset;

#[derive(Debug, Default)]
pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app //
            .init_asset::<LevelAsset>()
            .add_systems(Update, (new_level_bundle, level_bundle_loaded.map(error)));

        #[cfg(feature = "enable_reflect")]
        {
            use crate::level::LevelBundleLoadSettings;
            use crate::level::LevelComponent;
            app //
                .register_asset_reflect::<LevelAsset>()
                .register_type::<LevelBundleLoadSettings>()
                .register_type::<LevelComponent>();
        }
    }
}
