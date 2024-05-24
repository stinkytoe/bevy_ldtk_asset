use bevy::{prelude::*, utils::HashMap};
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum ToLoadError {
    #[error("Bad Parent Handle!")]
    BadParentHandle,
}

pub(crate) trait ToLoad<Parent: Asset, Child: Asset, GrandchildrenToLoad: Clone + Component>:
    Component + Sized
{
    fn to_load_changed_system(
        mut commands: Commands,
        to_load_changed_query: Query<(Entity, &Handle<Parent>, &Self), Changed<Self>>,
        loaded_query: Query<(Entity, &Handle<Child>)>,
        assets: Res<Assets<Parent>>,
    ) -> Result<(), ToLoadError> {
        for (entity, parent_handle, to_load) in to_load_changed_query.iter() {
            let parent_asset = assets
                .get(parent_handle)
                .ok_or(ToLoadError::BadParentHandle)?;

            let mut to_load = to_load.next_tier(parent_asset)?;

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

    fn merge_empty() -> Result<HashMap<Handle<Child>, GrandchildrenToLoad>, ToLoadError> {
        Ok(HashMap::default())
    }

    fn merge_filtered(
        ids: &HashMap<String, GrandchildrenToLoad>,
        assets_by_id: &HashMap<String, Handle<Child>>,
    ) -> Result<HashMap<Handle<Child>, GrandchildrenToLoad>, ToLoadError> {
        ids.iter()
            .map(|(id, levels_to_load)| {
                assets_by_id
                    .get(id)
                    .map(|handle| (handle.clone(), levels_to_load.clone()))
                    .ok_or(ToLoadError::BadParentHandle)
            })
            .collect::<Result<_, _>>()
    }

    fn merge_all(
        to_load: &GrandchildrenToLoad,
        assets_by_id: &HashMap<String, Handle<Child>>,
    ) -> Result<HashMap<Handle<Child>, GrandchildrenToLoad>, ToLoadError> {
        Ok(
            assets_by_id
                .values()
                .map(|handle| (handle.clone(), to_load.clone()))
                .collect(), // todo!(),
        )
    }

    fn next_tier(
        &self,
        // assets_by_id: &HashMap<String, Handle<Child>>,
        parent_asset: &Parent,
    ) -> Result<HashMap<Handle<Child>, GrandchildrenToLoad>, ToLoadError>;

    fn spawn_child(
        child_builder: &mut ChildBuilder,
        child_asset: Handle<Child>,
        to_load: GrandchildrenToLoad,
    );
}
