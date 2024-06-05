use bevy::prelude::*;
use bevy::utils::error;

use crate::layer::EntitiesToLoad;
use crate::layer::LayerAsset;
use crate::layer::Tile;
use crate::layer::Tiles;
use crate::traits::ChildrenEntityLoader;
use crate::traits::NewAssetEntitySystem;

use super::systems::handle_layer_tiles;

#[derive(Debug, Default)]
pub struct LayerPlugin;

impl Plugin for LayerPlugin {
    fn build(&self, app: &mut App) {
        app //
            .init_asset::<LayerAsset>()
            .register_asset_reflect::<LayerAsset>()
            .register_type::<Tile>()
            .register_type::<EntitiesToLoad>()
            .register_type::<Tiles>()
            .add_systems(
                Update,
                (
                    LayerAsset::new_asset_entity_system,
                    LayerAsset::bundle_loaded.map(error),
                    LayerAsset::asset_modified_or_removed_system.map(error),
                    LayerAsset::to_load_changed_system.map(error),
                    handle_layer_tiles.map(error),
                ),
            );
    }
}
