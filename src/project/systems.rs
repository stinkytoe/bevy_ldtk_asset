use bevy::{asset::LoadState, prelude::*};
use thiserror::Error;

use crate::project::LoadWorlds;
use crate::project::ProjectAsset;
use crate::project::ProjectBundleLoadSettings;
use crate::project::ProjectResolver;
use crate::world::WorldBundle;

#[derive(Component, Debug)]
pub(crate) struct ProjectBundleLoadStub;

#[derive(Debug, Error)]
pub enum NewProjectBundleError {
    #[error("Failed to load project asset after receiving LoadState::Loaded?")]
    ProjectAssetLoadFail,
    #[error("Bad world handle in project, or bad level iid!")]
    BadWorldIid,
    #[error("Couldn't get string representation of project path?")]
    BadProjectPath,
}

pub(crate) fn new_project_bundle(
    mut commands: Commands,
    new_project_query: Query<Entity, Added<ProjectBundleLoadSettings>>,
) {
    for entity in new_project_query.iter() {
        commands.entity(entity).insert(ProjectBundleLoadStub);
    }
}

pub(crate) fn project_bundle_loaded(
    mut commands: Commands,
    new_project_query: Query<
        (Entity, &Handle<ProjectAsset>, &ProjectBundleLoadSettings),
        With<ProjectBundleLoadStub>,
    >,
    asset_server: Res<AssetServer>,
    project_assets: Res<Assets<ProjectAsset>>,
) -> Result<(), NewProjectBundleError> {
    for (entity, project_handle, load_settings) in new_project_query.iter() {
        let Some(LoadState::Loaded) = asset_server.get_load_state(project_handle) else {
            return Ok(());
        };

        let project_asset = project_assets
            .get(project_handle)
            .ok_or(NewProjectBundleError::ProjectAssetLoadFail)?;

        debug!("ProjectAsset loaded! {:?}", project_handle.path());

        let mut entity_commands = commands.entity(entity);

        let worlds = project_asset
            .get_worlds()
            .filter(|world| match &load_settings.load_worlds {
                LoadWorlds::None => false,
                LoadWorlds::ByIdentifiers(ids) | LoadWorlds::ByIids(ids) => {
                    ids.contains(&world.identifier)
                }
                LoadWorlds::All => true,
            });

        for world in worlds {
            let world = project_asset
                .get_world_handle(&world.iid)
                .ok_or(NewProjectBundleError::BadWorldIid)?
                .clone();

            let settings = load_settings.world_bundle_load_settings.clone();

            entity_commands.with_children(move |parent| {
                parent.spawn(WorldBundle {
                    world,
                    settings,
                    spatial: SpatialBundle::default(),
                });
            });
        }

        entity_commands
            .insert(Name::from(format!(
                "{:?}",
                project_handle
                    .path()
                    .ok_or(NewProjectBundleError::BadProjectPath)?
            )))
            .remove::<ProjectBundleLoadStub>();
    }

    Ok(())
}
