use bevy::prelude::*;
use bevy::utils::error;

use crate::entity::EntityAsset;
use crate::traits::NewAssetEntitySystem;

#[derive(Debug, Default)]
pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app //
            .init_asset::<EntityAsset>()
            .register_asset_reflect::<EntityAsset>()
            .add_systems(
                Update,
                (
                    EntityAsset::new_asset_entity_system,
                    EntityAsset::bundle_loaded.map(error),
                ),
            );
    }
}
