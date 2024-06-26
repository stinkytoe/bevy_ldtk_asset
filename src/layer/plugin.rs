use bevy::prelude::*;
use bevy::utils::error;

use crate::layer::int_grid::IntGrid;
use crate::layer::systems::handle_layer_tiles;
use crate::layer::EntitiesToLoad;
use crate::layer::LayerAsset;
use crate::layer::Tile;
use crate::layer::Tiles;
use crate::traits::ChildrenEntityLoader;
use crate::traits::NewAssetEntitySystem;

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
            .register_type::<IntGrid>()
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
