use bevy_app::{App, Plugin};
use bevy_asset::AssetApp;

use crate::entity::Entity;
use crate::entity_definition::EntityDefinition;
use crate::enum_definition::EnumDefinition;
use crate::iid::Iid;
use crate::layer::Layer;
use crate::layer_definition::LayerDefinition;
use crate::level::{Level, LevelBackground};
use crate::project::Project;
use crate::project_loader::ProjectLoader;
use crate::tileset_definition::TilesetDefinition;
use crate::world::World;

/// The top level Bevy plugin!
///
/// Use this to enable the features of this plugin within your Bevy app!
///
/// See [Plugins](https://bevyengine.org/learn/quick-start/getting-started/plugins/)
/// from The Bevy Book for details.
#[derive(Debug)]
pub struct BevyLdtkAssetPlugin;

impl Plugin for BevyLdtkAssetPlugin {
    fn build(&self, app: &mut App) {
        app //
            .init_asset::<Entity>()
            .init_asset::<Layer>()
            .init_asset::<Level>()
            .init_asset::<Project>()
            .init_asset::<World>()
            .init_asset::<EntityDefinition>()
            .init_asset::<LayerDefinition>()
            .init_asset::<TilesetDefinition>()
            .init_asset::<EnumDefinition>()
            .init_asset_loader::<ProjectLoader>()
            .register_asset_reflect::<Entity>()
            .register_asset_reflect::<Layer>()
            .register_asset_reflect::<Level>()
            .register_asset_reflect::<Project>()
            .register_asset_reflect::<World>()
            .register_type::<Iid>()
            .register_type::<LevelBackground>();

        #[cfg(feature = "asset_events_debug")]
        {
            use bevy_app::Update;

            use crate::systems::asset_events_debug::*;

            app.add_systems(Update, (project_asset_events_debug_output,));
        }
    }
}
