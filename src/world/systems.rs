use bevy::prelude::*;
use thiserror::Error;

use crate::prelude::Iid;

use super::WorldAsset;

#[derive(Debug, Error)]
pub enum NewWorldAssetSystemError {
    #[error("Bad world handle!")]
    BadWorldHandle,
}

#[allow(clippy::type_complexity)]
pub(crate) fn new_world_asset(
    mut events: EventReader<AssetEvent<WorldAsset>>,
    mut commands: Commands,
    mut modified_world_query: Query<(
        Entity,
        &Handle<WorldAsset>,
        Option<&mut Name>,
        Option<&mut Iid>,
    )>,
    removed_world_query: Query<(Entity, &Handle<WorldAsset>)>,
    world_assets: Res<Assets<WorldAsset>>,
) -> Result<(), NewWorldAssetSystemError> {
    for event in events.read() {
        match event {
            AssetEvent::Added { id } | AssetEvent::LoadedWithDependencies { id } => {
                for (entity, world_handle, name, iid) in modified_world_query
                    .iter_mut()
                    .filter(|(_, handle, _, _)| handle.id() == *id)
                {
                    let project_asset = world_assets
                        .get(*id)
                        .ok_or(NewWorldAssetSystemError::BadWorldHandle)?;

                    let world_asset = world_assets
                        .get(*id)
                        .ok_or(NewWorldAssetSystemError::BadWorldHandle)?;

                    debug!("Adding/Modifying components for: {:?}", world_handle.path());

                    if let Some(mut name) = name {
                        name.set(world_asset.identifier.clone());
                    } else {
                        commands
                            .entity(entity)
                            .insert(Name::from(world_asset.identifier.clone()));
                    };

                    if let Some(mut iid) = iid {
                        iid.iid.clone_from(&project_asset.iid);
                    } else {
                        commands.entity(entity).insert(Iid {
                            iid: world_asset.iid.clone(),
                        });
                    };
                }
            }
            AssetEvent::Modified { id: _ } => {}
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
