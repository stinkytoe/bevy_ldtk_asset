use bevy::prelude::*;

use crate::common_components::Iid;
use crate::common_components::Size;
use crate::level::LayersToLoad;
use crate::level::LevelAsset;
use crate::prelude::EntitiesToLoad;
use crate::prelude::LayerAsset;
use crate::prelude::LayerBundle;
use crate::traits::ChildrenEntityLoader;
use crate::traits::NewAssetEntitySystem;

impl NewAssetEntitySystem for LevelAsset {
    type ModifiedQueryData = (
        &'static mut Name,
        &'static mut Iid,
        &'static mut Size,
        &'static mut Transform,
    );

    fn finalize(
        &self,
        mut entity_commands: bevy::ecs::system::EntityCommands,
    ) -> Result<(), crate::traits::NewAssetEntitySystemError> {
        entity_commands.insert((
            Name::new(self.identifier.clone()),
            Iid::new(self.iid.clone()),
            Size::new(self.size),
            Transform::from_translation(self.world_location),
        ));

        Ok(())
    }

    fn modify(
        &self,
        modified_query_result: crate::traits::ModifiedQueryResult<Self>,
    ) -> Result<(), crate::traits::NewAssetEntitySystemError> {
        let (mut name, mut iid, mut size, mut transform) = modified_query_result;

        *name = Name::new(self.identifier.clone());

        iid.iid.clone_from(&self.iid);

        size.size.clone_from(&self.size);

        *transform = Transform::from_translation(self.world_location);

        Ok(())
    }
}

impl ChildrenEntityLoader for LevelAsset {
    type Child = LayerAsset;
    type ChildrenToLoad = LayersToLoad;
    type GrandchildrenToLoad = EntitiesToLoad;

    fn next_tier(
        &self,
        to_load: &Self::ChildrenToLoad,
    ) -> Result<
        bevy::utils::HashMap<Handle<Self::Child>, Self::GrandchildrenToLoad>,
        crate::traits::ChildrenEntityLoaderError,
    > {
        match to_load {
            LayersToLoad::None => Self::merge_empty(),
            LayersToLoad::ByIdentifiers(ids) => {
                Self::merge_filtered(ids, &self.layer_assets_by_identifier)
            }
            LayersToLoad::ByIids(ids) => Self::merge_filtered(ids, &self.layer_assets_by_iid),
            LayersToLoad::TileLayersOnly => todo!(),
            LayersToLoad::EntityLayersOnly => todo!(),
            LayersToLoad::All(entities_to_load) => {
                Self::merge_all(entities_to_load, &self.layer_assets_by_iid)
            }
        }
    }

    fn spawn_child(
        child_builder: &mut ChildBuilder,
        layer: Handle<Self::Child>,
        entities_to_load: Self::GrandchildrenToLoad,
    ) {
        child_builder.spawn(LayerBundle {
            layer,
            entities_to_load,
            spatial: SpatialBundle::default(),
        });
    }
}
