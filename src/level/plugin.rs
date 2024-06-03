use bevy::prelude::*;
use bevy::utils::error;

use crate::level::LayersToLoad;
use crate::level::LevelAsset;
use crate::traits::ChildrenEntityLoader;
use crate::traits::NewAssetEntitySystem;
use crate::world::LevelsToLoad;

#[derive(Debug, Default)]
pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app //
            .init_asset::<LevelAsset>()
            .register_asset_reflect::<LevelAsset>()
            .register_type::<LevelsToLoad>()
            .register_type::<LayersToLoad>()
            .add_systems(
                Update,
                (
                    LevelAsset::new_asset_entity_system,
                    LevelAsset::bundle_loaded.map(error),
                    LevelAsset::to_load_changed_system.map(error),
                    LevelAsset::asset_modified_or_removed_system.map(error),
                ),
            );
    }
}
