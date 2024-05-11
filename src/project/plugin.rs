use bevy::prelude::*;

use crate::project::ProjectAsset;
use crate::project::ProjectAssetLoader;

#[derive(Debug, Default)]
pub struct ProjectPlugin;

impl Plugin for ProjectPlugin {
    fn build(&self, app: &mut App) {
        app //
            .init_asset::<ProjectAsset>()
            .init_asset_loader::<ProjectAssetLoader>();
    }
}
