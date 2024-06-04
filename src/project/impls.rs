use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use crate::common_components::Iid;
use crate::project::ProjectAsset;
use crate::traits::AssetProvidesProjectHandle;
use crate::traits::ModifiedQueryResult;
use crate::traits::NewAssetEntitySystem;
use crate::traits::NewAssetEntitySystemError;

impl AssetProvidesProjectHandle for ProjectAsset {
    fn project_handle(&self) -> Handle<ProjectAsset> {
        self.self_handle.clone()
    }
}

impl NewAssetEntitySystem for ProjectAsset {
    type ModifiedQueryData = (Entity, &'static mut Iid);

    fn finalize(
        &self,
        mut entity_commands: EntityCommands,
        _project_asset: &ProjectAsset,
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
        _entity_commands: EntityCommands,
        modified_query_result: ModifiedQueryResult<Self>,
        _project_asset: &ProjectAsset,
    ) -> Result<(), NewAssetEntitySystemError> {
        let (_entity, mut iid) = modified_query_result;

        if iid.iid != self.iid {
            iid.iid.clone_from(&self.iid);
        }

        Ok(())
    }
}
