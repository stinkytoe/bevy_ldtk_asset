use bevy::prelude::*;

use crate::{assets::project_loader::ProjectLoader, prelude::*};

pub struct BevyLdtkToolkitPlugin;

impl Plugin for BevyLdtkToolkitPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<ProjectAsset>()
            .init_asset_loader::<ProjectLoader>()
            .init_asset::<LdtkWorld>();
    }
}
