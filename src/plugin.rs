use bevy::prelude::*;
use bevy::utils::error;

use crate::level::respond_to_new_level_bundle;
use crate::level::LevelAsset;
use crate::project::ProjectAsset;
use crate::project::ProjectAssetLoader;
use crate::world::respond_to_new_world_bundle;
use crate::world::WorldAsset;

pub struct BevyLdtkLevelsPlugin;

impl Plugin for BevyLdtkLevelsPlugin {
    fn build(&self, app: &mut App) {
        app //
            .init_asset::<ProjectAsset>()
            .init_asset::<WorldAsset>()
            .init_asset::<LevelAsset>()
            .init_asset_loader::<ProjectAssetLoader>()
            .add_systems(
                Update,
                (
                    respond_to_new_level_bundle.map(error),
                    respond_to_new_world_bundle.map(error),
                ),
            );

        #[cfg(feature = "enable_reflect")]
        {
            use crate::layer::LayerComponent;
            use crate::layer::LoadEntityLayerSettings;
            use crate::layer::LoadTileLayerSettings;
            use crate::level::LevelBundleLoadSettings;
            use crate::level::LevelComponent;
            use crate::world::WorldBundleLoadSettings;
            use crate::world::WorldComponent;
            app //
                // .register_asset_reflect::<ProjectAsset>()
                .register_type::<LayerComponent>()
                .register_asset_reflect::<LevelAsset>()
                .register_asset_reflect::<WorldAsset>()
                .register_type::<LoadEntityLayerSettings>()
                .register_type::<LoadTileLayerSettings>()
                .register_type::<LevelBundleLoadSettings>()
                .register_type::<LevelComponent>()
                .register_type::<WorldBundleLoadSettings>()
                .register_type::<WorldComponent>();
        }
    }
}
