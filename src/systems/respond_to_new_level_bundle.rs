use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy::utils::thiserror;
use thiserror::Error;

use crate::prelude::LevelAsset;
use crate::prelude::LevelBundleLoadSettings;
use crate::prelude::LevelComponent;
use crate::prelude::LevelComponentError;
use crate::project::ProjectAsset;
use crate::project::ProjectResolver;

#[derive(Debug, Error)]
pub enum NewLevelBundleError {
    #[error("Failed to load level asset after receiving LoadState::Loaded?")]
    LevelAssetLoadFail,
    #[error("Project asset not loaded before world asset?")]
    ProjectAssetLoadFail,
    #[error("IID not found in project! {0}")]
    IidNotFound(String),
    #[error("LevelComponentError: {0}")]
    LevelComponentError(#[from] LevelComponentError),
    // #[error("Bad level handle in project, or bad level iid!")]
    // BadLevelIid,
}

pub(crate) fn respond_to_new_level_bundle(
    mut commands: Commands,
    new_level_query: Query<(Entity, &Handle<LevelAsset>, &LevelBundleLoadSettings)>,
    asset_server: Res<AssetServer>,
    level_assets: Res<Assets<LevelAsset>>,
    project_assets: Res<Assets<ProjectAsset>>,
) -> Result<(), NewLevelBundleError> {
    for (entity, id, load_settings) in new_level_query.iter() {
        if let Some(LoadState::Loaded) = asset_server.get_load_state(id) {
            debug!("LevelAsset loaded!");

            let level_asset = level_assets
                .get(id)
                .ok_or(NewLevelBundleError::LevelAssetLoadFail)?;

            let project_asset = project_assets
                .get(level_asset.project_handle.clone())
                .ok_or(NewLevelBundleError::ProjectAssetLoadFail)?;

            let level_json = project_asset
                .get_level_by_iid(&level_asset.iid)
                .ok_or(NewLevelBundleError::IidNotFound(level_asset.iid.clone()))?;

            let level_component: LevelComponent = level_json.try_into()?;

            let mut entity_commands = commands.entity(entity);

            if load_settings.load_bg_color {
                entity_commands.with_children(|parent| {
                    parent.spawn(Name::from("bg_color"));
                });
            }

            if load_settings.load_bg_image {
                entity_commands.with_children(|parent| {
                    parent.spawn(Name::from("bg_image"));
                });
            }

            {
                // let layers = level_json.layer_instances.iter();
            }

            entity_commands
                .insert((Name::from(level_component.identifier()), level_component))
                .remove::<LevelBundleLoadSettings>();
        }
    }
    //         debug!("WorldAsset loaded!");
    //
    //
    //         let mut entity_commands = commands.entity(entity);
    //
    //         // Level Children loading
    //         {
    //             let levels = project_asset
    //                 .get_levels_by_world_iid(world_component.iid())
    //                 .filter(|level| match &load_settings.load_levels {
    //                     crate::prelude::LoadLevels::None => false,
    //                     crate::prelude::LoadLevels::ByIdentifiers(ids)
    //                     | crate::prelude::LoadLevels::ByIids(ids) => {
    //                         ids.contains(&level.identifier)
    //                     }
    //                     crate::prelude::LoadLevels::All => true,
    //                 });
    //
    //             for level in levels {
    //                 let level = project_asset
    //                     .level_handles
    //                     .get(&level.iid)
    //                     .ok_or(NewWorldBundleError::BadLevelIid)?
    //                     .clone();
    //
    //                 let load_settings = load_settings.level_bundle_load_settings.clone();
    //
    //                 entity_commands.with_children(move |parent| {
    //                     parent.spawn(LevelBundle {
    //                         level,
    //                         load_settings,
    //                     });
    //                 });
    //             }
    //         }
    //
    //         entity_commands
    //             .insert((Name::from(world_component.identifier()), world_component))
    //             .remove::<WorldBundleLoadSettings>();
    //     }
    // }

    Ok(())
}
