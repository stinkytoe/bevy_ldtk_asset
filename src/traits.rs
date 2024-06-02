use bevy::ecs::query::QueryData;
use bevy::ecs::query::QueryEntityError;
use bevy::ecs::system::EntityCommands;
use bevy::{prelude::*, utils::HashMap};
use std::fmt::Debug;
use thiserror::Error;

use crate::project::ProjectAsset;

pub(crate) trait AssetProvidesProjectHandle: Asset {
    fn project_handle(&self) -> &Handle<ProjectAsset>;
}

#[derive(Debug, Error)]
pub(crate) enum ChildrenEntityLoaderError {
    #[error("Bad Self Handle!")]
    BadSelfHandle,
    #[error("Bad Project Handle!")]
    BadProjectHandle,
}

pub(crate) trait ChildrenEntityLoader: Asset + AssetProvidesProjectHandle + Sized {
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
    ) -> Result<(), ChildrenEntityLoaderError> {
        for (entity, self_handle, children_to_load) in changed_query.iter() {
            let self_asset = self_assets
                .get(self_handle)
                .ok_or(ChildrenEntityLoaderError::BadSelfHandle)?;

            let project_handle = self_asset.project_handle();

            let project_asset = project_assets
                .get(project_handle)
                .ok_or(ChildrenEntityLoaderError::BadProjectHandle)?;

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
    ) -> Result<HashMap<Handle<Self::Child>, Self::GrandchildrenToLoad>, ChildrenEntityLoaderError>
    {
        Ok(HashMap::default())
    }

    fn merge_filtered(
        ids: &HashMap<String, Self::GrandchildrenToLoad>,
        assets_by_id: &HashMap<String, Handle<Self::Child>>,
    ) -> Result<HashMap<Handle<Self::Child>, Self::GrandchildrenToLoad>, ChildrenEntityLoaderError>
    {
        ids.iter()
            .map(|(id, levels_to_load)| {
                assets_by_id
                    .get(id)
                    .map(|handle| (handle.clone(), levels_to_load.clone()))
                    .ok_or(ChildrenEntityLoaderError::BadSelfHandle)
            })
            .collect::<Result<_, _>>()
    }

    fn merge_all(
        to_load: &Self::GrandchildrenToLoad,
        assets_by_id: &HashMap<String, Handle<Self::Child>>,
    ) -> Result<HashMap<Handle<Self::Child>, Self::GrandchildrenToLoad>, ChildrenEntityLoaderError>
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
    ) -> Result<HashMap<Handle<Self::Child>, Self::GrandchildrenToLoad>, ChildrenEntityLoaderError>;

    fn spawn_child(
        child_builder: &mut ChildBuilder,
        child_asset: Handle<Self::Child>,
        to_load: Self::GrandchildrenToLoad,
    );
}

#[derive(Debug, Error)]
pub(crate) enum NewAssetEntitySystemError {
    #[error(transparent)]
    QueryEntityError(#[from] QueryEntityError),
    #[error("Bad handle!")]
    BadHandle,
    #[error("Finalize failed! {0}: {1}")]
    FailedFinalize(&'static str, &'static str),
    // #[error("Modify failed! {0}: {1}")]
    // FailedModify(&'static str, &'static str),
}

pub(crate) type ModifiedQueryResult<'a, T> =
    <<T as NewAssetEntitySystem>::ModifiedQueryData as bevy::ecs::query::WorldQuery>::Item<'a>;

pub(crate) trait NewAssetEntitySystem: Asset + Sized {
    type ModifiedQueryData: QueryData;

    fn new_asset_entity_system(
        mut commands: Commands,
        mut events: EventReader<AssetEvent<Self>>,
        entities_query: Query<(Entity, &Handle<Self>)>,
        mut modified_query: Query<Self::ModifiedQueryData>,
        assets: Res<Assets<Self>>,
    ) -> Result<(), NewAssetEntitySystemError> {
        for event in events.read() {
            match event {
                AssetEvent::Added { id } | AssetEvent::LoadedWithDependencies { id } => {
                    for (entity, handle) in entities_query
                        .iter()
                        .filter(|(_, handle)| handle.id() == *id)
                    {
                        let asset = assets
                            .get(handle)
                            .ok_or(NewAssetEntitySystemError::BadHandle)?;

                        let entity_commands = commands.entity(entity);

                        asset.finalize(entity_commands)?;
                    }
                }
                AssetEvent::Modified { id } => {
                    for (entity, handle) in entities_query
                        .iter()
                        .filter(|(_, handle)| handle.id() == *id)
                    {
                        let asset = assets
                            .get(handle)
                            .ok_or(NewAssetEntitySystemError::BadHandle)?;

                        let modified_query_data = modified_query.get_mut(entity)?;

                        asset.modify(modified_query_data)?;
                    }
                }
                AssetEvent::Removed { id } | AssetEvent::Unused { id } => {
                    for (entity, _) in entities_query
                        .iter()
                        .filter(|(_, handle)| handle.id() == *id)
                    {
                        commands.entity(entity).despawn_recursive();
                    }
                }
            };
        }

        Ok(())
    }

    fn finalize(&self, entity_commands: EntityCommands) -> Result<(), NewAssetEntitySystemError>;
    fn modify(
        &self,
        modified_query_result: ModifiedQueryResult<Self>,
    ) -> Result<(), NewAssetEntitySystemError>;
}
