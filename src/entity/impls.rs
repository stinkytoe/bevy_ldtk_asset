use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use crate::common_components::Iid;
use crate::entity::EntityAsset;
use crate::project::defs::TilesetRectangle;
use crate::traits::AssetProvidesProjectHandle;
use crate::traits::NewAssetEntitySystem;

impl AssetProvidesProjectHandle for EntityAsset {
    fn project_handle(&self) -> Handle<crate::prelude::ProjectAsset> {
        self.project.clone()
    }
}

impl NewAssetEntitySystem for EntityAsset {
    type ModifiedQueryData = (
        &'static mut Name,
        &'static mut Iid,
        Option<&'static mut TilesetRectangle>,
        &'static mut Transform,
    );

    fn finalize(
        &self,
        mut entity_commands: bevy::ecs::system::EntityCommands,
        _project_asset: &crate::prelude::ProjectAsset,
    ) -> Result<(), crate::traits::NewAssetEntitySystemError> {
        entity_commands.insert((
            Name::new(self.identifier.clone()),
            Iid::new(self.iid.clone()),
            Transform::from_translation(self.location.extend(0.0)),
        ));

        if let Some(tile) = &self.tile {
            entity_commands.insert(tile.clone());
        }

        Ok(())
    }

    fn modify(
        &self,
        mut entity_commands: EntityCommands,
        modified_query_result: crate::traits::ModifiedQueryResult<Self>,
        _project_asset: &crate::prelude::ProjectAsset,
    ) -> Result<(), crate::traits::NewAssetEntitySystemError> {
        let (mut name, mut iid, tile, mut transform) = modified_query_result;

        *name = Name::new(self.identifier.clone());

        iid.iid.clone_from(&self.iid);

        match (tile, self.tile.as_ref()) {
            (None, None) => (),
            (_, Some(asset_tile)) => {
                entity_commands.insert(asset_tile.clone());
            }
            (Some(_), None) => {
                entity_commands.remove::<TilesetRectangle>();
            } // (Some(tile), Some(asset_tile)) => ,
        };

        transform.translation = self.location.extend(0.0);

        Ok(())
    }
}
