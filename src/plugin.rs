use bevy::prelude::*;

use crate::prelude::LevelAsset;
use crate::project::ProjectAsset;
use crate::project::ProjectAssetLoader;
use crate::world::WorldAsset;

pub struct BevyLdtkLevelsPlugin;

impl Plugin for BevyLdtkLevelsPlugin {
    fn build(&self, app: &mut App) {
        app //
            .init_asset::<ProjectAsset>()
            .init_asset::<WorldAsset>()
            .init_asset::<LevelAsset>()
            .init_asset_loader::<ProjectAssetLoader>();
    }
}
