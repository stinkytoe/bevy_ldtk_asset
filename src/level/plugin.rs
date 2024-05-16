use bevy::prelude::*;

use crate::level::LevelAsset;

#[derive(Debug, Default)]
pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app //
            .init_asset::<LevelAsset>()
            .register_asset_reflect::<LevelAsset>();
    }
}
