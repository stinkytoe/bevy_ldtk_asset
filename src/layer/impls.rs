use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use crate::common_components::Iid;
use crate::entity::EntityAsset;
use crate::entity::EntityBundle;
use crate::layer::EntitiesToLoad;
use crate::layer::LayerAsset;
use crate::layer::Tiles;
use crate::project::ProjectAsset;
use crate::traits::AssetProvidesProjectHandle;
use crate::traits::ChildrenEntityLoader;
use crate::traits::NewAssetEntitySystem;
use crate::traits::NilToLoad;

impl AssetProvidesProjectHandle for LayerAsset {
    fn project_handle(&self) -> Handle<ProjectAsset> {
        self.project.clone()
    }
}

impl NewAssetEntitySystem for LayerAsset {
    type ModifiedQueryData = (
        &'static mut Name,
        &'static mut Iid,
        &'static mut Tiles,
        &'static mut Transform,
    );

    fn finalize(
        &self,
        mut entity_commands: bevy::ecs::system::EntityCommands,
        project_asset: &ProjectAsset,
    ) -> Result<(), crate::traits::NewAssetEntitySystemError> {
        let settings = &project_asset.settings;

        entity_commands.insert((
            Name::new(self.identifier.clone()),
            Iid::new(self.identifier.clone()),
            Tiles {
                tiles: self.tiles.clone(),
            },
            Transform::from_translation(
                self.px_total_offset
                    .as_vec2()
                    .extend((self.index + 2) as f32 * settings.layer_separation),
            ),
        ));

        Ok(())
    }

    fn modify(
        &self,
        _entity_commands: EntityCommands,
        modified_query_result: crate::traits::ModifiedQueryResult<Self>,
        project_asset: &ProjectAsset,
    ) -> Result<(), crate::traits::NewAssetEntitySystemError> {
        let (mut name, mut iid, mut tiles, mut transform) = modified_query_result;

        let settings = &project_asset.settings;

        *name = Name::new(self.identifier.clone());

        iid.iid.clone_from(&self.iid);

        tiles.tiles.clone_from(&self.tiles);

        transform.translation = self
            .px_total_offset
            .as_vec2()
            .extend((self.index + 2) as f32 * settings.layer_separation);

        Ok(())
    }
}

impl ChildrenEntityLoader for LayerAsset {
    type Child = EntityAsset;
    type ChildrenToLoad = EntitiesToLoad;
    type GrandchildrenToLoad = NilToLoad;

    fn next_tier(
        &self,
        to_load: &Self::ChildrenToLoad,
    ) -> Result<
        bevy::utils::HashMap<Handle<Self::Child>, Self::GrandchildrenToLoad>,
        crate::traits::ChildrenEntityLoaderError,
    > {
        match to_load {
            EntitiesToLoad::None => Self::merge_empty(),
            EntitiesToLoad::ByIdentifiers(ids) => {
                Self::merge_filtered(ids, &self.entity_assets_by_identifier)
            }
            EntitiesToLoad::ByIids(ids) => Self::merge_filtered(ids, &self.entity_assets_by_iid),
            EntitiesToLoad::All => Self::merge_all(&NilToLoad, &self.entity_assets_by_iid),
        }
    }

    fn spawn_child(
        child_builder: &mut ChildBuilder,
        world: Handle<Self::Child>,
        _to_load: Self::GrandchildrenToLoad,
    ) {
        child_builder.spawn(EntityBundle {
            world,
            spatial: SpatialBundle::default(),
        });
    }
}
