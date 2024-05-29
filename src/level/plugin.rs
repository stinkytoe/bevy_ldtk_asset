use bevy::prelude::*;

use crate::level::LevelAsset;
use crate::prelude::LevelsToLoad;

#[derive(Debug, Default)]
pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app //
            .init_asset::<LevelAsset>()
            .register_asset_reflect::<LevelAsset>()
            .register_type::<LevelsToLoad>()
            // .add_systems(
            //     Update,
            //     (
                    // new_level_asset.map(error),
                    // LevelsToLoad::to_load_changed_system.map(error),
                // ),
            //)
        ;
    }
}
