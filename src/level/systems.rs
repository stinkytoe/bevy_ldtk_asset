// use bevy::prelude::*;
// use thiserror::Error;

// #[derive(Debug, Error)]
// pub enum NewLevelAssetSystemError {
// #[error("Bad level handle!")]
// BadLevelHandle,
// }

// #[allow(clippy::type_complexity)]
// pub(crate) fn new_level_asset(
//     mut events: EventReader<AssetEvent<LevelAsset>>,
//     mut commands: Commands,
//     mut modified_asset_query: Query<(
//         Entity,
//         &Handle<LevelAsset>,
//         Option<&mut LevelBackgroundPosition>,
//         Option<&mut Neighbours>,
//         Option<&mut FieldInstances>,
//         Option<&mut Name>,
//         Option<&mut Iid>,
//         Option<&mut Size>,
//         &mut Transform,
//     )>,
//     removed_asset_query: Query<(Entity, &Handle<LevelAsset>)>,
//     level_assets: Res<Assets<LevelAsset>>,
// ) -> Result<(), NewLevelAssetSystemError> {
//     for event in events.read() {
//         match event {
//             AssetEvent::Added { id } | AssetEvent::LoadedWithDependencies { id } => {
//                 // for (
//                 //     entity,
//                 //     level_handle,
//                 //     level_background_position,
//                 //     neighbours,
//                 //     field_instances,
//                 //     name,
//                 //     iid,
//                 //     size,
//                 //     transform,
//                 // ) in modified_asset_query
//                 //     .iter_mut()
//                 //     .filter(|(_, handle, _, _, _, _, _, _, _)| handle.id() == *id)
//                 // {
//                 //     let level_asset = level_assets
//                 //         .get(*id)
//                 //         .ok_or(NewLevelAssetSystemError::BadLevelHandle)?;
//                 //
//                 //     debug!("Adding/Modifying components for: {:?}", level_handle.path());
//                 //
//                 //     match (level_background_position, level_asset.bg_pos.as_ref()) {
//                 //         (None, None) => {}
//                 //         (None, Some(bg_pos)) => {
//                 //             commands.entity(entity).insert(bg_pos.clone());
//                 //         }
//                 //         (Some(_), None) => {
//                 //             commands.entity(entity).remove::<LevelBackgroundPosition>();
//                 //         }
//                 //         (Some(mut level_background_position), Some(bg_pos)) => {
//                 //             *level_background_position = bg_pos.clone();
//                 //         }
//                 //     }
//                 // }
//             }
//             AssetEvent::Modified { id: _ } => {}
//             AssetEvent::Removed { id } | AssetEvent::Unused { id } => {
//                 for (entity, _) in removed_asset_query
//                     .iter()
//                     .filter(|(_, handle)| handle.id() == *id)
//                 {
//                     commands.entity(entity).despawn_recursive();
//                 }
//             }
//         }
//     }
//     Ok(())
// }
