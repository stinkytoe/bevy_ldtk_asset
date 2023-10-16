use crate::{assets::ldtk_project_loader::LdtkRootLoader, ldtk_project::LdtkProject, systems};
use bevy::prelude::*;

pub struct LdtkBevyLoaderPlugin;

impl Plugin for LdtkBevyLoaderPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app //
            .add_asset::<LdtkProject>()
            .add_asset_loader(LdtkRootLoader)
            .add_systems(
                Update,
                (
                    systems::load_level_backgrounds::check_for_levels_which_need_images_loaded,
                    systems::load_level_backgrounds::load_level_backgrounds,
                ),
            );
    }
}
