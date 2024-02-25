use bevy::prelude::*;

use crate::{assets::project_loader::ProjectLoader, prelude::*};

pub struct BevyLdtkAssetPlugin;

impl Plugin for BevyLdtkAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<ProjectAsset>()
            .init_asset_loader::<ProjectLoader>()
            .init_asset::<LdtkWorld>();
    }
}
