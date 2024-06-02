use bevy::ecs::query::{QueryData, QueryEntityError};
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use crate::common_components::Iid;
use crate::project::ProjectAsset;
use crate::traits::NewAssetEntitySystemError;
use crate::traits::{ModifiedQueryResult, NewAssetEntitySystem};

impl NewAssetEntitySystem for ProjectAsset {
    type ModifiedQueryData = (Entity, &'static mut Iid);

    fn finalize(
        &self,
        mut entity_commands: EntityCommands,
    ) -> Result<(), NewAssetEntitySystemError> {
        entity_commands.insert((
            Name::from(
                self.self_handle
                    .path()
                    .ok_or(NewAssetEntitySystemError::FailedFinalize(
                        "ProjectAsset",
                        "bad self_handle?",
                    ))?
                    .to_string(),
            ),
            Iid {
                iid: self.iid.clone(),
            },
        ));

        Ok(())
    }

    fn modify(
        &self,
        modified_query_result: ModifiedQueryResult<Self>,
    ) -> Result<(), NewAssetEntitySystemError> {
        let (_entity, mut iid) = modified_query_result;

        if iid.iid != self.iid {
            iid.iid.clone_from(&self.iid);
        }

        Ok(())
    }
}
