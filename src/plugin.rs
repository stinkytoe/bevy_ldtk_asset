use bevy::prelude::*;

use crate::{
    assets::project_loader::ProjectAssetLoader, prelude::*,
    systems::process_load_parameters::process_load_parameters,
};

pub struct BevyLdtkAssetPlugin;

impl Plugin for BevyLdtkAssetPlugin {
    fn build(&self, app: &mut App) {
        app //
            .init_asset::<ProjectAsset>()
            .init_asset::<WorldAsset>()
            .init_asset::<LevelAsset>()
            .init_asset_loader::<ProjectAssetLoader>()
            .add_systems(Update, process_load_parameters);
    }
}
