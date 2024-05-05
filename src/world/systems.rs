use bevy::asset::LoadState;
use bevy::prelude::*;
use thiserror::Error;

use crate::level::LevelBundle;
use crate::project::ProjectAsset;
use crate::project::ProjectResolver;
use crate::world::LoadLevels;
use crate::world::WorldAsset;
use crate::world::WorldBundleLoadSettings;
use crate::world::WorldComponent;

#[derive(Component, Debug)]
pub(crate) struct WorldBundleLoadStub;

#[derive(Debug, Error)]
pub enum NewWorldBundleError {
    #[error("Failed to load world asset after receiving LoadState::Loaded?")]
    WorldAssetLoadFail,
    #[error("Project asset not loaded before world asset?")]
    ProjectAssetLoadFail,
    #[error("IID not found in project! {0}")]
    IidNotFound(String),
    #[error("Bad level handle in project, or bad level iid!")]
    BadLevelIid,
}

pub(crate) fn new_world_bundle(
    mut commands: Commands,
    new_world_query: Query<Entity, Added<WorldBundleLoadSettings>>,
) {
    for entity in &new_world_query {
        commands.entity(entity).insert(WorldBundleLoadStub);
    }
}

pub(crate) fn world_bundle_loaded(
    mut commands: Commands,
    new_world_query: Query<
        (Entity, &Handle<WorldAsset>, &WorldBundleLoadSettings),
        With<WorldBundleLoadStub>,
    >,
    asset_server: Res<AssetServer>,
    world_assets: Res<Assets<WorldAsset>>,
    project_assets: Res<Assets<ProjectAsset>>,
) -> Result<(), NewWorldBundleError> {
    for (entity, world_handle, load_settings) in new_world_query.iter() {
        let Some(LoadState::Loaded) = asset_server.get_load_state(world_handle) else {
            return Ok(());
        };

        let world_asset = world_assets
            .get(world_handle)
            .ok_or(NewWorldBundleError::WorldAssetLoadFail)?;

        // This is probably paranoia
        let Some(LoadState::Loaded) = asset_server.get_load_state(&world_asset.project_handle)
        else {
            return Ok(());
        };

        let project_asset = project_assets
            .get(&world_asset.project_handle)
            .ok_or(NewWorldBundleError::ProjectAssetLoadFail)?;

        debug!("WorldAsset loaded! {:?}", world_handle.path());

        let world_component: WorldComponent = project_asset
            .get_world_by_iid(&world_asset.iid)
            .ok_or(NewWorldBundleError::IidNotFound(world_asset.iid.clone()))?
            .into();

        let mut entity_commands = commands.entity(entity);

        // Level Children loading
        {
            let levels = project_asset
                .get_levels_by_world_iid(world_component.iid())
                .filter(|level| match &load_settings.load_levels {
                    LoadLevels::None => false,
                    LoadLevels::ByIdentifiers(ids) | LoadLevels::ByIids(ids) => {
                        ids.contains(&level.identifier)
                    }
                    LoadLevels::All => true,
                });

            for level in levels {
                let level = project_asset
                    .get_level_handle(&level.iid)
                    .ok_or(NewWorldBundleError::BadLevelIid)?
                    .clone();

                let load_settings = load_settings.level_bundle_load_settings.clone();

                entity_commands.with_children(move |parent| {
                    parent.spawn(LevelBundle {
                        level,
                        load_settings,
                        spatial: SpatialBundle::default(),
                    });
                });
            }
        }

        entity_commands
            .insert((Name::from(world_component.identifier()), world_component))
            .remove::<WorldBundleLoadStub>();
    }

    Ok(())
}
