use bevy::prelude::*;
use bevy::utils::error;

use crate::project::WorldsToLoad;
use crate::traits::ChildrenEntityLoader;
use crate::traits::NewAssetEntitySystem;
use crate::world::LevelsToLoad;
use crate::world::WorldAsset;

#[derive(Debug, Default)]
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app //
            .init_asset::<WorldAsset>()
            .register_asset_reflect::<WorldAsset>()
            .register_type::<WorldsToLoad>()
            .register_type::<LevelsToLoad>()
            .add_systems(
                Update,
                (
                    WorldAsset::new_asset_entity_system,
                    WorldAsset::bundle_loaded.map(error),
                    WorldAsset::asset_modified_or_removed_system.map(error),
                    WorldAsset::to_load_changed_system.map(error),
                ),
            );
    }
}
