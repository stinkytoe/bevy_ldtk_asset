use bevy::prelude::*;
use bevy::utils::error;

use crate::project::ProjectAsset;
use crate::project::ProjectAssetLoader;

use crate::project::new_project_bundle;
use crate::project::project_bundle_loaded;

#[derive(Debug, Default)]
pub struct ProjectPlugin;

impl Plugin for ProjectPlugin {
    fn build(&self, app: &mut App) {
        app //
            .init_asset::<ProjectAsset>()
            .init_asset_loader::<ProjectAssetLoader>()
            .add_systems(
                Update,
                (new_project_bundle, project_bundle_loaded.map(error)),
            );
    }
}
