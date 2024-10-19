use bevy::prelude::*;

use crate::entity::Entity;
use crate::iid::Iid;
use crate::layer::Layer;
use crate::level::Level;
use crate::project::Project;
use crate::project_loader::ProjectLoader;
use crate::world::World;

#[derive(Debug)]
pub struct BevyLdtkAssetPlugin;

impl Plugin for BevyLdtkAssetPlugin {
    fn build(&self, app: &mut App) {
        app //
            .init_asset::<Project>()
            .init_asset::<World>()
            .init_asset::<Level>()
            .init_asset::<Layer>()
            .init_asset::<Entity>()
            .init_asset_loader::<ProjectLoader>()
            .register_asset_reflect::<Project>()
            .register_asset_reflect::<World>()
            .register_asset_reflect::<Level>()
            .register_asset_reflect::<Layer>()
            .register_asset_reflect::<Entity>()
            .register_type::<Iid>();

        #[cfg(feature = "asset_events_debug")]
        {
            use crate::systems::asset_events_debug::*;
            app.add_systems(
                Update,
                (
                    ldtk_project_debug_output,
                    ldtk_asset_debug_output::<Entity>,
                    ldtk_asset_debug_output::<Layer>,
                    ldtk_asset_debug_output::<Level>,
                    ldtk_asset_debug_output::<World>,
                ),
            );
        }
    }
}
