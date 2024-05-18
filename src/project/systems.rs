use bevy::prelude::*;
use thiserror::Error;

use crate::level::LevelsToLoad;
use crate::prelude::Iid;
use crate::project::ProjectAsset;
use crate::project::ProjectComponent;
use crate::world::WorldAsset;
use crate::world::WorldBundle;
use crate::world::WorldsToLoad;

#[derive(Debug, Error)]
pub enum NewProjectAssetSystemError {
    #[error("Bad project handle!")]
    BadProjectHandle,
    #[error("Bad project asset path!")]
    BadProjectAssetPath,
    // #[error("Requested world not found! field: {0:?}, value: {1:?}")]
    // WorldNotFound(String, String),
}

#[allow(clippy::self_assignment)]
pub(crate) fn new_project_asset(
    mut events: EventReader<AssetEvent<ProjectAsset>>,
    mut commands: Commands,
    new_project_query: Query<(Entity, &Handle<ProjectAsset>)>,
    mut modified_project_query: Query<(&Handle<ProjectAsset>, &mut Iid)>,
    removed_project_query: Query<(Entity, &Handle<ProjectAsset>)>,
    project_assets: Res<Assets<ProjectAsset>>,
    // asset_server: Res<AssetServer>,
) -> Result<(), NewProjectAssetSystemError> {
    for event in events.read() {
        // debug!("event: {event:?}");
        match event {
            AssetEvent::Added { id } | AssetEvent::LoadedWithDependencies { id } => {
                for (entity, project_handle) in new_project_query
                    .iter()
                    .filter(|(_, handle)| handle.id() == *id)
                {
                    let project_asset = project_assets
                        .get(*id)
                        .ok_or(NewProjectAssetSystemError::BadProjectHandle)?;

                    commands.entity(entity).insert((
                        Name::from(
                            project_handle
                                .path()
                                .ok_or(NewProjectAssetSystemError::BadProjectAssetPath)?
                                .to_string(),
                        ),
                        Iid {
                            iid: project_asset.iid.clone(),
                        },
                    ));
                }
            }
            AssetEvent::Modified { id } => {
                for (project_handle, mut iid) in modified_project_query
                    .iter_mut()
                    .filter(|(handle, _)| handle.id() == *id)
                {
                    let project_asset = project_assets
                        .get(*id)
                        .ok_or(NewProjectAssetSystemError::BadProjectHandle)?;

                    iid.iid.clone_from(&project_asset.iid);
                }
            }
            AssetEvent::Removed { id } | AssetEvent::Unused { id } => {
                for (entity, _) in removed_project_query
                    .iter()
                    .filter(|(_, handle)| handle.id() == *id)
                {
                    commands.entity(entity).despawn_recursive();
                }
            }
        }
    }
    // for (entity, mut project_handle, worlds_to_load) in query.iter() {
    //     if let Some(LoadState::Loaded) = asset_server.get_load_state(project_handle) {
    //         let project_asset = project_assets
    //             .get(project_handle)
    //             .ok_or(NewProjectAssetSystemError::BadProjectHandle)?;
    //
    //         commands.entity(entity).insert((
    //             Name::from(format!(
    //                 "{:?}",
    //                 project_handle
    //                     .path()
    //                     .ok_or(NewProjectAssetSystemError::BadProjectAssetPath)?
    //             )),
    //             ProjectComponent {},
    //         ));
    //     }
    // }

    Ok(())
}

#[derive(Debug, Error)]
pub enum ChangedProjectAssetSystemError {
    #[error("Bad project handle!")]
    BadProjectHandle,
}

#[allow(clippy::type_complexity)]
pub(crate) fn changed_project_asset(
    mut commands: Commands,
    project_query: Query<
        (Entity, &Handle<ProjectAsset>, &WorldsToLoad),
        (
            With<ProjectComponent>,
            Or<(Changed<WorldsToLoad>, Changed<Handle<ProjectAsset>>)>,
        ),
    >,
    worlds_query: Query<(Entity, &Handle<WorldAsset>)>,
    project_assets: Res<Assets<ProjectAsset>>,
) -> Result<(), ChangedProjectAssetSystemError> {
    for (entity, project_handle, worlds_to_load) in project_query.iter() {
        debug!("changed: {project_handle:?}");
        let project_asset = project_assets
            .get(project_handle)
            .ok_or(ChangedProjectAssetSystemError::BadProjectHandle)?;

        let worlds_to_load: Vec<_> = match worlds_to_load {
            WorldsToLoad::None => vec![],
            WorldsToLoad::ByIdentifiers(identifiers) => identifiers
                .iter()
                .filter_map(|(identifier, levels_to_load)| {
                    project_asset
                        .world_assets_by_iid
                        .get(identifier)
                        .map(|handle| (handle, levels_to_load.clone()))
                })
                .collect(),
            WorldsToLoad::ByIids(iids) => iids
                .iter()
                .filter_map(|(iid, levels_to_load)| {
                    project_asset
                        .world_assets_by_iid
                        .get(iid)
                        .map(|handle| (handle, levels_to_load.clone()))
                })
                .collect(),
            WorldsToLoad::All => project_asset
                .world_assets_by_iid
                .values()
                .map(|handle| (handle, LevelsToLoad::All))
                .collect(),
        };

        for (world_entity, world_handle) in worlds_query.iter() {
            // if worlds_to_load.iter().any()
        }

        for (world, levels_to_load) in worlds_to_load {
            commands.entity(entity).with_children(|parent| {
                parent.spawn(WorldBundle {
                    world: world.clone(),
                    levels_to_load,
                    spatial: SpatialBundle::default(),
                });
            });
        }
    }

    Ok(())
}
