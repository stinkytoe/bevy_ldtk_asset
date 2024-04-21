use bevy::prelude::*;

use crate::prelude::LevelAsset;
use crate::prelude::WorldBundleLoadSettings;
use crate::project::ProjectAsset;
use crate::project::ProjectAssetLoader;
use crate::systems::respond_to_new_world_bundle;
use crate::world::WorldAsset;

pub struct BevyLdtkLevelsPlugin;

impl Plugin for BevyLdtkLevelsPlugin {
    fn build(&self, app: &mut App) {
        app //
            .init_asset::<ProjectAsset>()
            .init_asset::<WorldAsset>()
            .init_asset::<LevelAsset>()
            .init_asset_loader::<ProjectAssetLoader>()
            .add_systems(Update, respond_to_new_world_bundle);

        #[cfg(feature = "enable_reflect")]
        app //
            //.register_asset_reflect::<ProjectAsset>()
            .register_asset_reflect::<WorldAsset>()
            .register_asset_reflect::<LevelAsset>()
            .register_type::<WorldBundleLoadSettings>();
    }
}
