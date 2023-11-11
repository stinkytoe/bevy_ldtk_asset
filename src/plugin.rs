use crate::assets::ldtk_level::LdtkLevel;
use crate::assets::ldtk_level_loader::LdtkLevelLoader;
use crate::assets::ldtk_project::LdtkProject;
use crate::assets::ldtk_project_loader::LdtkProjectLoader;
use bevy::prelude::*;

pub struct BevyLdtkAssetPlugin;

impl Plugin for BevyLdtkAssetPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app //
            .init_asset::<LdtkProject>()
            .init_asset_loader::<LdtkProjectLoader>()
            .init_asset::<LdtkLevel>()
            .init_asset_loader::<LdtkLevelLoader>();
    }
}
