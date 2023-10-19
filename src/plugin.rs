use crate::assets::ldtk_level::LdtkLevel;
use crate::assets::ldtk_level_loader::LdtkLevelLoader;
use crate::assets::ldtk_project::LdtkProject;
use crate::assets::ldtk_project_loader::LdtkRootLoader;
use crate::systems::world_set_changed::world_set_changed;
use crate::systems::LdtkSet;
use bevy::prelude::*;

pub struct LdtkBevyLoaderPlugin;

impl Plugin for LdtkBevyLoaderPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app //
            .add_asset::<LdtkProject>()
            .add_asset_loader(LdtkRootLoader)
            .add_asset::<LdtkLevel>()
            .add_asset_loader(LdtkLevelLoader)
            .add_systems(Update, (world_set_changed).in_set(LdtkSet));
    }
}
