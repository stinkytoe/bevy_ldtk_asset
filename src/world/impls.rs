use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use crate::common_components::Iid;
use crate::level::LayersToLoad;
use crate::level::LevelAsset;
use crate::level::LevelBundle;
use crate::project::ProjectAsset;
use crate::traits::AssetProvidesProjectHandle;
use crate::traits::ChildrenEntityLoader;
use crate::traits::NewAssetEntitySystem;
use crate::world::LevelsToLoad;
use crate::world::WorldAsset;

impl AssetProvidesProjectHandle for WorldAsset {
    fn project_handle(&self) -> Handle<ProjectAsset> {
        self.project.clone()
    }
}

impl NewAssetEntitySystem for WorldAsset {
    type ModifiedQueryData = (&'static mut Name, &'static mut Iid);

    fn finalize(
        &self,
        mut entity_commands: bevy::ecs::system::EntityCommands,
        _project_asset: &ProjectAsset,
    ) -> Result<(), crate::traits::NewAssetEntitySystemError> {
        entity_commands.insert((
            Name::from(self.identifier.clone()),
            Iid::new(self.iid.clone()),
        ));
        Ok(())
    }

    fn modify(
        &self,
        _entity_commands: EntityCommands,
        modified_query_result: crate::traits::ModifiedQueryResult<Self>,
        _project_asset: &ProjectAsset,
    ) -> Result<(), crate::traits::NewAssetEntitySystemError> {
        let (mut name, mut iid) = modified_query_result;

        *name = Name::new(self.identifier.clone());

        iid.iid.clone_from(&self.iid);

        Ok(())
    }
}

impl ChildrenEntityLoader for WorldAsset {
    type Child = LevelAsset;
    type ChildrenToLoad = LevelsToLoad;
    type GrandchildrenToLoad = LayersToLoad;

    fn next_tier(
        &self,
        to_load: &Self::ChildrenToLoad,
    ) -> Result<
        bevy::utils::HashMap<Handle<Self::Child>, Self::GrandchildrenToLoad>,
        crate::traits::ChildrenEntityLoaderError,
    > {
        match to_load {
            LevelsToLoad::None => Self::merge_empty(),
            LevelsToLoad::ByIdentifiers(ids) => {
                Self::merge_filtered(ids, &self.level_assets_by_identifier)
            }
            LevelsToLoad::ByIids(ids) => Self::merge_filtered(ids, &self.level_assets_by_iid),
            LevelsToLoad::All(levels_to_load) => {
                Self::merge_all(levels_to_load, &self.level_assets_by_iid)
            }
        }
    }

    fn spawn_child(
        child_builder: &mut ChildBuilder,
        level: Handle<Self::Child>,
        layers_to_load: Self::GrandchildrenToLoad,
    ) {
        child_builder.spawn(LevelBundle {
            level,
            layers_to_load,
            spatial: SpatialBundle::default(),
        });
    }
}
