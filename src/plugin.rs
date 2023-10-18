use crate::assets::ldtk_level::LdtkLevel;
use crate::assets::ldtk_level_loader::LdtkLevelLoader;
use crate::assets::ldtk_project::LdtkProject;
use crate::assets::ldtk_project_loader::LdtkRootLoader;
use crate::systems::load_world_event::update_load_world_event;
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
            .add_systems(Update, (update_load_world_event,).in_set(LdtkSet));
    }
}
