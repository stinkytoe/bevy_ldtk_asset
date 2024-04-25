use bevy::prelude::*;
use bevy::utils::error;

use crate::world::respond_to_new_world_bundle;
use crate::world::WorldAsset;

#[derive(Debug, Default)]
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app //
            .init_asset::<WorldAsset>()
            .add_systems(Update, respond_to_new_world_bundle.map(error));

        #[cfg(feature = "enable_reflect")]
        {
            use crate::world::WorldBundleLoadSettings;
            use crate::world::WorldComponent;
            app //
                .register_type::<WorldBundleLoadSettings>()
                .register_type::<WorldComponent>();
        }
    }
}
