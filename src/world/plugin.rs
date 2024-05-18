use bevy::prelude::*;

use crate::world::WorldAsset;
use crate::world::WorldsToLoad;

#[derive(Debug, Default)]
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app //
            .init_asset::<WorldAsset>()
            .register_asset_reflect::<WorldAsset>()
            .register_type::<WorldsToLoad>();
    }
}
