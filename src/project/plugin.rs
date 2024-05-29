use bevy::prelude::*;
use bevy::utils::error;

use crate::project::new_project_asset;
use crate::project::ProjectAsset;
use crate::project::ProjectAssetLoader;
use crate::traits::DependencyLoader;

// use super::systems::ToLoad;
// use super::systems::WorldsToLoad2;

#[derive(Debug, Default)]
pub struct ProjectPlugin;

impl Plugin for ProjectPlugin {
    fn build(&self, app: &mut App) {
        app //
            .init_asset::<ProjectAsset>()
            .init_asset_loader::<ProjectAssetLoader>()
            .register_asset_reflect::<ProjectAsset>()
            .add_systems(
                Update,
                (
                    new_project_asset.map(error),
                    ProjectAsset::to_load_changed_system.map(error),
                    // project_asset_worlds_to_load_changed.map(error),
                    // WorldsToLoad2::to_load_changed_system.map(error),
                ),
            );
    }
}
