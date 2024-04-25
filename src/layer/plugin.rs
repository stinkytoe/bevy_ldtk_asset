use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct LayerPlugin;

impl Plugin for LayerPlugin {
    fn build(&self, _app: &mut App) {
        #[cfg(feature = "enable_reflect")]
        {
            use crate::layer::LayerComponent;
            use crate::layer::LoadEntityLayerSettings;
            use crate::layer::LoadTileLayerSettings;
            _app //
                // .register_asset_reflect::<ProjectAsset>()
                .register_type::<LayerComponent>()
                .register_type::<LoadEntityLayerSettings>()
                .register_type::<LoadTileLayerSettings>();
        }
    }
}
