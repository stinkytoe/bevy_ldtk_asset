use bevy_app::{App, Plugin};
use bevy_asset::AssetApp;

use crate::entity::Entity;
use crate::iid::Iid;
use crate::layer::Layer;
use crate::layer_definition::LayerDefinition;
use crate::level::{Level, LevelBackground};
use crate::project::Project;
use crate::project_loader::ProjectLoader;
use crate::tileset_definition::TilesetDefinition;
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
            .init_asset::<TilesetDefinition>()
            .init_asset::<LayerDefinition>()
            .init_asset_loader::<ProjectLoader>()
            .register_asset_reflect::<Project>()
            .register_asset_reflect::<World>()
            .register_asset_reflect::<Level>()
            .register_asset_reflect::<Layer>()
            .register_asset_reflect::<Entity>()
            .register_type::<Iid>()
            .register_type::<LevelBackground>();

        #[cfg(feature = "asset_events_debug")]
        {
            use bevy_app::Update;

            use crate::systems::asset_events_debug::*;

            app.add_systems(
                Update,
                (
                    project_asset_events_debug_output,
                    ldtk_asset_events_debug_output::<Entity>,
                    ldtk_asset_events_debug_output::<Layer>,
                    ldtk_asset_events_debug_output::<Level>,
                    ldtk_asset_events_debug_output::<World>,
                ),
            );
        }
    }
}
