use bevy::prelude::*;

use crate::{
    assets::project_loader::ProjectAssetLoader, prelude::*,
    systems::process_load_parameters::process_load_parameters,
};

/// The plugin which the user should include in their main function
/// during app creation to enable all of bevy_ldtk_asset's features
pub struct BevyLdtkAssetPlugin;

impl Plugin for BevyLdtkAssetPlugin {
    fn build(&self, app: &mut App) {
        app //
            .init_asset::<ProjectAsset>()
            .init_asset::<WorldAsset>()
            .init_asset::<LevelAsset>()
            .init_asset_loader::<ProjectAssetLoader>()
            // .add_systems(Update, process_load_parameters_world)
            .add_systems(Update, process_load_parameters::<WorldAsset>)
            .add_systems(Update, process_load_parameters::<LevelAsset>);
    }
}
