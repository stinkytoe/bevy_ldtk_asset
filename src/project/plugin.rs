use bevy::prelude::*;
use bevy::utils::error;

use crate::project::changed_project_asset;
use crate::project::new_project_asset;
use crate::project::ProjectAsset;
use crate::project::ProjectAssetLoader;

#[derive(Debug, Default)]
pub struct ProjectPlugin;

impl Plugin for ProjectPlugin {
    fn build(&self, app: &mut App) {
        app //
            .init_asset::<ProjectAsset>()
            .init_asset_loader::<ProjectAssetLoader>()
            .register_asset_reflect::<ProjectAsset>();

        app.add_systems(
            Update,
            (
                new_project_asset.map(error),
                changed_project_asset.map(error),
            ),
        );
    }
}
