use bevy::prelude::*;
use std::fmt::Debug;
use thiserror::Error;

use crate::common_components::Iid;
use crate::project::ProjectAsset;

#[derive(Debug, Error)]
pub enum NewProjectAssetSystemError {
    #[error("Bad project handle!")]
    BadProjectHandle,
    #[error("Bad project asset path!")]
    BadProjectAssetPath,
}

pub(crate) fn new_project_asset(
    mut events: EventReader<AssetEvent<ProjectAsset>>,
    mut commands: Commands,
    new_project_query: Query<(Entity, &Handle<ProjectAsset>)>,
    mut modified_project_query: Query<(&Handle<ProjectAsset>, &mut Iid)>,
    removed_project_query: Query<(Entity, &Handle<ProjectAsset>)>,
    project_assets: Res<Assets<ProjectAsset>>,
) -> Result<(), NewProjectAssetSystemError> {
    for event in events.read() {
        match event {
            AssetEvent::Added { id } | AssetEvent::LoadedWithDependencies { id } => {
                for (entity, project_handle) in new_project_query
                    .iter()
                    .filter(|(_, handle)| handle.id() == *id)
                {
                    let project_asset = project_assets
                        .get(*id)
                        .ok_or(NewProjectAssetSystemError::BadProjectHandle)?;

                    debug!("Adding components for: {:?}", project_handle.path());

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

                    debug!("Modifying components for: {:?}", project_handle.path());

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

    Ok(())
}
