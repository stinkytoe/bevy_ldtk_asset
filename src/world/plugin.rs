use bevy::prelude::*;
use bevy::utils::error;

use crate::traits::ToLoad;
use crate::world::WorldAsset;
use crate::world::WorldsToLoad;

use crate::world::new_world_asset;

#[derive(Debug, Default)]
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app //
            .init_asset::<WorldAsset>()
            .register_asset_reflect::<WorldAsset>()
            .register_type::<WorldsToLoad>()
            .add_systems(
                Update,
                (
                    new_world_asset.map(error),
                    WorldsToLoad::to_load_changed_system.map(error),
                ),
            );
    }
}
