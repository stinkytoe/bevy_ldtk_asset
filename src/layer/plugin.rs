use bevy::prelude::*;

use crate::layer::LayerAsset;
use crate::layer::Tile;
use crate::layer::Tiles;

#[derive(Debug, Default)]
pub struct LayerPlugin;

impl Plugin for LayerPlugin {
    fn build(&self, app: &mut App) {
        app //
            .init_asset::<LayerAsset>()
            .register_asset_reflect::<LayerAsset>()
            .register_type::<Tile>()
            .register_type::<Tiles>();
    }
}
