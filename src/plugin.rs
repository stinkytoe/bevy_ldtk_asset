use crate::assets::ldtk_level_loader::LdtkLevelLoader;
use crate::assets::ldtk_project_loader::LdtkProjectLoader;
use crate::ldtk::level_asset::LevelAsset;
use crate::ldtk::project::Project;
use crate::resources::LdtkLevels;
use crate::systems::level_asset_loading::{levels_changed, process_level_loading};
use bevy::prelude::*;

/// The bevy plugin for enabling the features of this crate.
/// See [The Bevy Book -- Plugins](https://bevyengine.org/learn/book/getting-started/plugins/)
pub struct BevyLdtkAssetPlugin;

impl Plugin for BevyLdtkAssetPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app //
            .init_asset::<Project>()
            .init_asset_loader::<LdtkProjectLoader>()
            .init_asset::<LevelAsset>()
            .init_asset_loader::<LdtkLevelLoader>()
            .init_resource::<LdtkLevels>()
            .add_systems(Update, process_level_loading)
            .add_systems(
                Update,
                levels_changed.run_if(resource_changed::<LdtkLevels>()),
            );
    }
}
