use bevy::prelude::*;
use thiserror::Error;

use crate::prelude::Iid;

use super::WorldAsset;

#[derive(Debug, Error)]
pub enum NewWorldAssetSystemError {
    #[error("Bad world handle!")]
    BadWorldHandle,
    // #[error("Bad project asset path!")]
    // BadProjectAssetPath,
}

pub(crate) fn new_world_asset(
    mut events: EventReader<AssetEvent<WorldAsset>>,
    mut commands: Commands,
    new_world_query: Query<(Entity, &Handle<WorldAsset>)>,
    mut modified_world_query: Query<(&Handle<WorldAsset>, &mut Iid)>,
    removed_world_query: Query<(Entity, &Handle<WorldAsset>)>,
    world_assets: Res<Assets<WorldAsset>>,
) -> Result<(), NewWorldAssetSystemError> {
    for event in events.read() {
        match event {
            AssetEvent::Added { id } | AssetEvent::LoadedWithDependencies { id } => {
                for (entity, world_handle) in new_world_query
                    .iter()
                    .filter(|(_, handle)| handle.id() == *id)
                {
                    let world_asset = world_assets
                        .get(*id)
                        .ok_or(NewWorldAssetSystemError::BadWorldHandle)?;

                    debug!("Adding components for: {:?}", world_handle.path());

                    commands.entity(entity).insert((
                        Name::from(world_asset.identifier.clone()),
                        Iid {
                            iid: world_asset.iid.clone(),
                        },
                    ));
                }
            }
            AssetEvent::Modified { id } => {
                for (world_handle, mut iid) in modified_world_query
                    .iter_mut()
                    .filter(|(handle, _)| handle.id() == *id)
                {
                    let project_asset = world_assets
                        .get(*id)
                        .ok_or(NewWorldAssetSystemError::BadWorldHandle)?;

                    debug!("Modifying components for: {:?}", world_handle.path());

                    iid.iid.clone_from(&project_asset.iid);
                }
            }
            AssetEvent::Removed { id } | AssetEvent::Unused { id } => {
                for (entity, _) in removed_world_query
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
