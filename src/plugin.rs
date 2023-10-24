use crate::assets::ldtk_level::LdtkLevel;
use crate::assets::ldtk_level_loader::LdtkLevelLoader;
use crate::assets::ldtk_project::LdtkProject;
use crate::assets::ldtk_project_loader::LdtkRootLoader;
use crate::components::LdtkRoot;
use crate::prelude::LdtkSet;
use crate::systems::update_loaded_ldtk_project::update_loaded_ldtk_project;
use bevy::prelude::*;

pub struct LdtkBevyLoaderPlugin;

impl Plugin for LdtkBevyLoaderPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app //
            .register_type::<LdtkRoot>()
            .init_asset::<LdtkProject>()
            .init_asset_loader::<LdtkRootLoader>()
            .init_asset::<LdtkLevel>()
            .init_asset_loader::<LdtkLevelLoader>()
            // .add_systems(Update, (add_children_once_assets_loaded).in_set(LdtkSet))
            .add_systems(Update, (update_loaded_ldtk_project).in_set(LdtkSet));
    }
}
