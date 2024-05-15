use bevy::prelude::*;
use bevy::utils::error;

use crate::layer::new_entity_layer_bundle;
use crate::layer::new_tile_layer_bundle;
use crate::layer::new_tiles;

#[derive(Debug, Default)]
pub struct LayerPlugin;

impl Plugin for LayerPlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_systems(
                Update,
                (
                    new_entity_layer_bundle.map(error),
                    new_tile_layer_bundle.map(error),
                    new_tiles.map(error),
                ),
            );

        #[cfg(feature = "enable_reflect")]
        {
            use crate::layer::LayerComponent;
            use crate::layer::LoadEntityLayerSettings;
            use crate::layer::LoadTileLayerMeshSettings;
            use crate::layer::LoadTileLayerSettings;
            use crate::layer::Tile;
            use crate::layer::Tiles;

            app //
                // .register_asset_reflect::<ProjectAsset>()
                .register_type::<LayerComponent>()
                .register_type::<LoadEntityLayerSettings>()
                .register_type::<LoadTileLayerMeshSettings>()
                .register_type::<LoadTileLayerSettings>()
                .register_type::<Tile>()
                .register_type::<Tiles>();
        }
    }
}
