use bevy::prelude::*;
use bevy::utils::error;

use crate::level::LevelAsset;
use crate::traits::DependencyLoader;
use crate::world::LevelsToLoad;

#[derive(Debug, Default)]
pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app //
            .init_asset::<LevelAsset>()
            .register_asset_reflect::<LevelAsset>()
            .register_type::<LevelsToLoad>()
            .add_systems(
                Update,
                (
                    // new_level_asset.map(error),
                    // LevelsToLoad::to_load_changed_system.map(error),
                    LevelAsset::to_load_changed_system.map(error),
                ),
            );
    }
}
