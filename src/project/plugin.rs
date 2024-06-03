use bevy::prelude::*;
use bevy::utils::error;

use crate::project::defs::EntityDefinition;
use crate::project::defs::EnumDefinition;
use crate::project::defs::LayerDefinition;
use crate::project::defs::TilesetDefinition;
use crate::project::ProjectAsset;
use crate::project::ProjectAssetLoader;
use crate::project::WorldsToLoad;
use crate::traits::ChildrenEntityLoader;
use crate::traits::NewAssetEntitySystem;

#[derive(Debug, Default)]
pub struct ProjectPlugin;

impl Plugin for ProjectPlugin {
    fn build(&self, app: &mut App) {
        app //
            .init_asset::<ProjectAsset>()
            .init_asset_loader::<ProjectAssetLoader>()
            .register_asset_reflect::<ProjectAsset>()
            .register_type::<LayerDefinition>()
            .register_type::<EntityDefinition>()
            .register_type::<TilesetDefinition>()
            .register_type::<EnumDefinition>()
            .register_type::<WorldsToLoad>()
            .add_systems(
                Update,
                (
                    ProjectAsset::new_asset_entity_system,
                    ProjectAsset::bundle_loaded.map(error),
                    ProjectAsset::asset_modified_or_removed_system.map(error),
                    ProjectAsset::to_load_changed_system.map(error),
                ),
            );
    }
}
