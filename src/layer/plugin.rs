use bevy::prelude::*;
use bevy::utils::error;

use crate::layer::new_tile_layer_bundle;

#[derive(Debug, Default)]
pub struct LayerPlugin;

impl Plugin for LayerPlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_systems(Update, new_tile_layer_bundle.map(error));

        #[cfg(feature = "enable_reflect")]
        {
            use crate::layer::LayerComponent;
            use crate::layer::LoadEntityLayerSettings;
            use crate::layer::LoadTileLayerSettings;
            app //
                // .register_asset_reflect::<ProjectAsset>()
                .register_type::<LayerComponent>()
                .register_type::<LoadEntityLayerSettings>()
                .register_type::<LoadTileLayerSettings>();
        }
    }
}
