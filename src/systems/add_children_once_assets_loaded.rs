use crate::assets::ldtk_level::LdtkLevel;
use crate::assets::ldtk_project::LdtkProject;
use crate::components::*;
use bevy::prelude::*;

pub(crate) fn add_children_once_assets_loaded(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    level_file_assets: Res<Assets<LdtkLevel>>,
    root_query: Query<(Entity, &WorldSet, &LevelSet, &LdtkRoot), Without<AssetsLoadedTag>>,
) {
    for (entity, _world_set, level_set, root) in root_query.iter() {
        let Some(project) = ldtk_project_assets.get(&root.project) else {
            debug!("No project, still loading?");
            return;
        };

        let level_background_handles: Vec<Handle<Image>> = project
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
            .cloned()
            .collect();

        let tileset_handles: Vec<Handle<Image>> = project.tilesets.values().cloned().collect();

        let level_file_handles: Vec<Handle<LdtkLevel>> = project
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
            .cloned()
            .collect();

        match asset_server.get_group_load_state(
            [root.project.id()]
                .iter()
                .cloned()
                .chain(level_background_handles.iter().map(|handle| handle.id()))
                .chain(tileset_handles.iter().map(|handle| handle.id()))
                .chain(level_file_handles.iter().map(|handle| handle.id())),
        ) {
            bevy::asset::LoadState::NotLoaded | bevy::asset::LoadState::Loading => {
                trace!("still loading...")
            }
            bevy::asset::LoadState::Loaded => {
                debug!("project, background, tileset image files, and level files loaded!");

                for level_file_handle in level_file_handles {
                    let Some(_level) = level_file_assets.get(&level_file_handle) else {
                        debug!("");
                        return;
                    };
                }

                commands.entity(entity).insert(AssetsLoadedTag);

                // commands.entity(entity).with_children(|parent| {
                //         for pro
                //     });
            }
            bevy::asset::LoadState::Failed => warn!("failed to load an image!"),
            bevy::asset::LoadState::Unloaded => debug!("all images unloaded?"),
        };
    }
}
