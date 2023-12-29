use crate::assets::ldtk_level::LdtkLevel;
use crate::assets::ldtk_level_loader::LdtkLevelLoader;
use crate::assets::ldtk_project::LdtkProject;
use crate::assets::ldtk_project_loader::LdtkProjectLoader;
use crate::resources::LdtkLevels;
use crate::systems::level_asset_loading::{levels_changed, process_level_loading};
use bevy::prelude::*;

pub struct BevyLdtkAssetPlugin;

impl Plugin for BevyLdtkAssetPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app //
            .init_asset::<LdtkProject>()
            .init_asset_loader::<LdtkProjectLoader>()
            .init_asset::<LdtkLevel>()
            .init_asset_loader::<LdtkLevelLoader>()
            .init_resource::<LdtkLevels>()
            // .init_resource::<LdtkProjects>()
            .add_systems(Update, process_level_loading)
            // .add_systems(Update, process_world_loading)
            .add_systems(
                Update,
                levels_changed.run_if(resource_changed::<LdtkLevels>()),
            );
    }
}
