use bevy::prelude::*;

use crate::prelude::{LdtkProject, LdtkRoot, LevelSet, RenderTag, WorldSet};

pub(crate) fn render_set_from_images(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    root_query: Query<(Entity, &WorldSet, &LevelSet, &LdtkRoot), Without<RenderTag>>,
) {
    for (entity, _world_set, level_set, root) in root_query.iter() {
        let Some(project) = ldtk_project_assets.get(&root.project) else {
            debug!("No project, still loading?");
            return;
        };

        let level_backgrounds = project
            .level_backgrounds
            .iter()
            .filter_map(|(iid, handle)| match level_set {
                LevelSet::All => Some(handle),
                LevelSet::Only(set) => {
                    if set.contains(iid) {
                        Some(handle)
                    } else {
                        None
                    }
                }
            })
            .map(|handle| handle.id());

        let level_file_handles = project
            .level_file_handles
            .iter()
            .filter_map(|(iid, handle)| match level_set {
                LevelSet::All => Some(handle),
                LevelSet::Only(set) => {
                    if set.contains(iid) {
                        Some(handle)
                    } else {
                        None
                    }
                }
            })
            .map(|handle| handle.id());

        let tilesets = project.tilesets.values().map(|handle| handle.id());

        match asset_server
            .get_group_load_state(level_backgrounds.chain(tilesets).chain(level_file_handles))
        {
            bevy::asset::LoadState::NotLoaded | bevy::asset::LoadState::Loading => {
                trace!("still loading...")
            }
            bevy::asset::LoadState::Loaded => {
                debug!("background and tileset image files loaded!");
                commands
                    .entity(entity)
                    .insert(RenderTag)
                    .with_children(|_parent| {
                        // for
                    });
            }
            bevy::asset::LoadState::Failed => warn!("failed to load an image!"),
            bevy::asset::LoadState::Unloaded => debug!("all images unloaded?"),
        };
    }
}
