use bevy::{prelude::*, utils::HashMap};
use std::fmt::Debug;
use thiserror::Error;

use crate::project::ProjectAsset;

pub(crate) trait AssetProvidesProjectHandle: Asset {
    fn project_handle(&self) -> &Handle<ProjectAsset>;
}

#[derive(Debug, Error)]
pub(crate) enum DependencyLoaderError {
    #[error("Bad Self Handle!")]
    BadSelfHandle,
    #[error("Bad Project Handle!")]
    BadProjectHandle,
}

pub(crate) trait DependencyLoader: Asset + AssetProvidesProjectHandle + Sized {
    type Child: Asset;
    type ChildrenToLoad: Clone + Component + Debug;
    type GrandchildrenToLoad: Clone + Component + Debug;

    #[allow(clippy::type_complexity)]
    fn to_load_changed_system(
        mut commands: Commands,
        changed_query: Query<
            (Entity, &Handle<Self>, &Self::ChildrenToLoad),
            Changed<Self::ChildrenToLoad>,
        >,
        loaded_query: Query<(Entity, &Handle<Self::Child>)>,
        project_assets: Res<Assets<ProjectAsset>>,
        self_assets: Res<Assets<Self>>,
    ) -> Result<(), DependencyLoaderError> {
        for (entity, self_handle, children_to_load) in changed_query.iter() {
            let self_asset = self_assets
                .get(self_handle)
                .ok_or(DependencyLoaderError::BadSelfHandle)?;

            let project_handle = self_asset.project_handle();

            let project_asset = project_assets
                .get(project_handle)
                .ok_or(DependencyLoaderError::BadProjectHandle)?;

            let mut to_load = self_asset.next_tier(project_asset, children_to_load)?;

            for (entity, child_handle) in loaded_query.iter() {
                if to_load.get(child_handle).is_some() {
                    to_load.remove(child_handle);
                } else {
                    commands.entity(entity).despawn_recursive();
                }
            }

            for (child_handle, grandchildren_to_load) in to_load.iter() {
                commands.entity(entity).with_children(|parent| {
                    Self::spawn_child(parent, child_handle.clone(), grandchildren_to_load.clone());
                });
            }
        }

        Ok(())
    }

    fn merge_empty(
    ) -> Result<HashMap<Handle<Self::Child>, Self::GrandchildrenToLoad>, DependencyLoaderError>
    {
        Ok(HashMap::default())
    }

    fn merge_filtered(
        ids: &HashMap<String, Self::GrandchildrenToLoad>,
        assets_by_id: &HashMap<String, Handle<Self::Child>>,
    ) -> Result<HashMap<Handle<Self::Child>, Self::GrandchildrenToLoad>, DependencyLoaderError>
    {
        ids.iter()
            .map(|(id, levels_to_load)| {
                assets_by_id
                    .get(id)
                    .map(|handle| (handle.clone(), levels_to_load.clone()))
                    .ok_or(DependencyLoaderError::BadSelfHandle)
            })
            .collect::<Result<_, _>>()
    }

    fn merge_all(
        to_load: &Self::GrandchildrenToLoad,
        assets_by_id: &HashMap<String, Handle<Self::Child>>,
    ) -> Result<HashMap<Handle<Self::Child>, Self::GrandchildrenToLoad>, DependencyLoaderError>
    {
        Ok(
            assets_by_id
                .values()
                .map(|handle| (handle.clone(), to_load.clone()))
                .collect(), // todo!(),
        )
    }

    fn next_tier(
        &self,
        project_asset: &ProjectAsset,
        to_load: &Self::ChildrenToLoad,
    ) -> Result<HashMap<Handle<Self::Child>, Self::GrandchildrenToLoad>, DependencyLoaderError>;

    fn spawn_child(
        child_builder: &mut ChildBuilder,
        child_asset: Handle<Self::Child>,
        to_load: Self::GrandchildrenToLoad,
    );
}

// pub(crate) trait ToLoad<Parent: Asset, Child: Asset, GrandchildrenToLoad: Clone + Component>:
//     Component + Sized
// {
//     fn to_load_changed_system(
//         mut commands: Commands,
//         to_load_changed_query: Query<(Entity, &Handle<Parent>, &Self), Changed<Self>>,
//         loaded_query: Query<(Entity, &Handle<Child>)>,
//         assets: Res<Assets<Parent>>,
//     ) -> Result<(), ToLoadError> {
//         for (entity, parent_handle, to_load) in to_load_changed_query.iter() {
//             let parent_asset = assets
//                 .get(parent_handle)
//                 .ok_or(ToLoadError::BadSelfHandle)?;
//
//             let mut to_load = to_load.next_tier(parent_asset)?;
//
//             for (entity, child_handle) in loaded_query.iter() {
//                 if to_load.get(child_handle).is_some() {
//                     to_load.remove(child_handle);
//                 } else {
//                     commands.entity(entity).despawn_recursive();
//                 }
//             }
//
//             for (child_handle, grandchildren_to_load) in to_load.iter() {
//                 commands.entity(entity).with_children(|parent| {
//                     Self::spawn_child(parent, child_handle.clone(), grandchildren_to_load.clone());
//                 });
//             }
//         }
//
//         Ok(())
//     }
//
//     fn merge_empty() -> Result<HashMap<Handle<Child>, GrandchildrenToLoad>, ToLoadError> {
//         Ok(HashMap::default())
//     }
//
//     fn merge_filtered(
//         ids: &HashMap<String, GrandchildrenToLoad>,
//         assets_by_id: &HashMap<String, Handle<Child>>,
//     ) -> Result<HashMap<Handle<Child>, GrandchildrenToLoad>, ToLoadError> {
//         ids.iter()
//             .map(|(id, levels_to_load)| {
//                 assets_by_id
//                     .get(id)
//                     .map(|handle| (handle.clone(), levels_to_load.clone()))
//                     .ok_or(ToLoadError::BadSelfHandle)
//             })
//             .collect::<Result<_, _>>()
//     }
//
//     fn merge_all(
//         to_load: &GrandchildrenToLoad,
//         assets_by_id: &HashMap<String, Handle<Child>>,
//     ) -> Result<HashMap<Handle<Child>, GrandchildrenToLoad>, ToLoadError> {
//         Ok(
//             assets_by_id
//                 .values()
//                 .map(|handle| (handle.clone(), to_load.clone()))
//                 .collect(), // todo!(),
//         )
//     }
//
//     fn next_tier(
//         &self,
//         // assets_by_id: &HashMap<String, Handle<Child>>,
//         parent_asset: &Parent,
//     ) -> Result<HashMap<Handle<Child>, GrandchildrenToLoad>, ToLoadError>;
//
//     fn spawn_child(
//         child_builder: &mut ChildBuilder,
//         child_asset: Handle<Child>,
//         to_load: GrandchildrenToLoad,
//     );
// }
