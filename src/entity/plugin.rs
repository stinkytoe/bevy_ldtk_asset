use bevy::prelude::*;

use crate::entity::EntityAsset;

#[derive(Debug, Default)]
pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app //
            .init_asset::<EntityAsset>()
            .register_asset_reflect::<EntityAsset>();
    }
}
