use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy::utils::thiserror;
use thiserror::Error;

use crate::prelude::WorldComponent;
use crate::project::ProjectAsset;
use crate::project::ProjectResolver;
use crate::world::WorldAsset;
use crate::world::WorldBundleLoadSettings;

#[derive(Debug, Error)]
pub enum NewWorldBundleError {
    #[error("Failed to load world asset after receiving LoadState::Loaded?")]
    WorldAssetLoadFail,
    #[error("Project asset not loaded before world asset?")]
    ProjectAssetLoadFail,
    #[error("IID not found in project! {0}")]
    IidNotFound(String),
}

pub(crate) fn respond_to_new_world_bundle(
    mut commands: Commands,
    new_world_query: Query<(Entity, &Handle<WorldAsset>, &WorldBundleLoadSettings)>,
    asset_server: Res<AssetServer>,
    world_assets: Res<Assets<WorldAsset>>,
    project_assets: Res<Assets<ProjectAsset>>,
) -> Result<(), NewWorldBundleError> {
    for (entity, id, _load_settings) in new_world_query.iter() {
        if let Some(LoadState::Loaded) = asset_server.get_load_state(id) {
            debug!("WorldAsset loaded!");

            let world_asset = world_assets
                .get(id)
                .ok_or(NewWorldBundleError::WorldAssetLoadFail)?;

            let project_asset = project_assets
                .get(world_asset.project_handle.clone())
                .ok_or(NewWorldBundleError::ProjectAssetLoadFail)?;

            let world_component: WorldComponent = project_asset
                .get_world_by_iid(&world_asset.iid)
                .ok_or(NewWorldBundleError::IidNotFound(world_asset.iid.clone()))?
                .into();

            commands
                .entity(entity)
                .insert((Name::from(world_component.identifier()), world_component))
                .remove::<WorldBundleLoadSettings>();
        }
    }

    Ok(())
}
