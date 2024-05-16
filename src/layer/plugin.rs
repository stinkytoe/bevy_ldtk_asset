use bevy::prelude::*;

use crate::layer::LayerAsset;

#[derive(Debug, Default)]
pub struct LayerPlugin;

impl Plugin for LayerPlugin {
    fn build(&self, app: &mut App) {
        app //
            .init_asset::<LayerAsset>()
            .register_asset_reflect::<LayerAsset>();
    }
}
