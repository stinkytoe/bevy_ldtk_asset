use bevy::prelude::*;

use crate::prelude::{LdtkProject, LdtkRoot, RenderSet};

pub(crate) fn render_set_from_images(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    root_query: Query<(Entity, &LdtkRoot), Without<RenderSet>>,
) {
    for (entity, root) in root_query.iter() {
        let Some(project) = ldtk_project_assets.get(&root.project) else {
            debug!("No project, still loading?");
            return;
        };

        let level_backgrounds = project.level_backgrounds.values().map(|handle| handle.id());
        let tilesets = project._tilesets.values().map(|handle| handle.id());

        match asset_server.get_group_load_state(level_backgrounds.chain(tilesets)) {
            bevy::asset::LoadState::NotLoaded | bevy::asset::LoadState::Loading => {
                trace!("still loading...")
            }
            bevy::asset::LoadState::Loaded => {
                debug!("background and tileset image files loaded!");
                commands.entity(entity).insert(RenderSet);
            }
            bevy::asset::LoadState::Failed => warn!("failed to load an image!"),
            bevy::asset::LoadState::Unloaded => debug!("all images unloaded?"),
        };
    }
}
