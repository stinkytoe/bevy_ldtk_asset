use bevy::prelude::*;

use crate::prelude::LevelAsset;
use crate::project::ProjectAsset;
use crate::project::ProjectAssetLoader;
use crate::systems::respond_to_new_world_bundle;
use crate::world::WorldAsset;
use bevy::utils::error;

pub struct BevyLdtkLevelsPlugin;

impl Plugin for BevyLdtkLevelsPlugin {
    fn build(&self, app: &mut App) {
        app //
            .init_asset::<ProjectAsset>()
            .init_asset::<WorldAsset>()
            .init_asset::<LevelAsset>()
            .init_asset_loader::<ProjectAssetLoader>()
            .add_systems(Update, respond_to_new_world_bundle.map(error));

        #[cfg(feature = "enable_reflect")]
        {
            use crate::prelude::WorldBundleLoadSettings;
            use crate::prelude::WorldComponent;
            app //
                //.register_asset_reflect::<ProjectAsset>()
                .register_asset_reflect::<WorldAsset>()
                .register_asset_reflect::<LevelAsset>()
                .register_type::<WorldComponent>()
                .register_type::<WorldBundleLoadSettings>();
        }
    }
}
