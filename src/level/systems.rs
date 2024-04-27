use bevy::asset::LoadState;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::utils::thiserror;
use thiserror::Error;

use crate::layer::EntityLayerBundle;
use crate::layer::LayerComponent;
use crate::layer::LayerComponentError;
use crate::layer::LayerType;
use crate::layer::TileLayerBundle;
use crate::level::LevelAsset;
use crate::level::LevelBundleLoadSettings;
use crate::level::LevelComponent;
use crate::level::LevelComponentError;
use crate::level::LoadLayers;
use crate::project::ProjectAsset;

#[derive(Component, Debug)]
pub(crate) struct LevelBundleLoadStub;

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
    #[error("LayerComponentError: {0}")]
    LayerComponentError(#[from] LayerComponentError),
    // #[error("Bad level handle in project, or bad level iid!")]
    // BadLevelIid,
}

pub(crate) fn new_level_bundle(
    mut commands: Commands,
    new_world_query: Query<Entity, Added<LevelBundleLoadSettings>>,
) {
    for entity in &new_world_query {
        commands.entity(entity).insert(LevelBundleLoadStub);
    }
}

pub(crate) fn level_bundle_loaded(
    mut commands: Commands,
    new_level_query: Query<
        (Entity, &Handle<LevelAsset>, &LevelBundleLoadSettings),
        With<LevelBundleLoadStub>,
    >,
    asset_server: Res<AssetServer>,
    level_assets: Res<Assets<LevelAsset>>,
    project_assets: Res<Assets<ProjectAsset>>,
) -> Result<(), NewLevelBundleError> {
    for (entity, level_handle, load_settings) in new_level_query.iter() {
        let Some(LoadState::Loaded) = asset_server.get_load_state(level_handle) else {
            return Ok(());
        };

        let level_asset = level_assets
            .get(level_handle)
            .ok_or(NewLevelBundleError::LevelAssetLoadFail)?;

        // This is probably paranoia
        let Some(LoadState::Loaded) = asset_server.get_load_state(&level_asset.project_handle)
        else {
            return Ok(());
        };

        let project_asset = project_assets
            .get(level_asset.project_handle.clone())
            .ok_or(NewLevelBundleError::ProjectAssetLoadFail)?;

        debug!("LevelAsset loaded! {:?}", level_handle.path());

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

        let spawn_tile_layer = move |entity_commands: &mut EntityCommands, layer| {
            entity_commands.with_children(|parent| {
                parent.spawn(TileLayerBundle {
                    project: level_asset.project_handle.clone(),
                    layer,
                    settings: load_settings.load_tile_layer_settings.clone(),
                    spatial: SpatialBundle::default(),
                });
            });
        };

        let spawn_entity_layer = move |entity_commands: &mut EntityCommands, layer| {
            entity_commands.with_children(|parent| {
                parent.spawn(EntityLayerBundle {
                    project: level_asset.project_handle.clone(),
                    layer,
                    settings: load_settings.load_entity_layer_settings.clone(),
                    spatial: SpatialBundle::default(),
                });
            });
        };

        if let Some(layer_instances) = level_json.layer_instances.as_ref() {
            for layer_json in layer_instances.iter().rev() {
                let layer: LayerComponent = layer_json.try_into()?;

                match (&load_settings.load_layers, layer.layer_type()) {
                    // Tile layer variants
                    (
                        LoadLayers::ByIdentifiers(ids),
                        LayerType::Tiles | LayerType::IntGrid | LayerType::Autolayer,
                    ) if ids.contains(&layer_json.identifier) => {
                        spawn_tile_layer(&mut entity_commands, layer);
                    }
                    (
                        LoadLayers::ByIids(ids),
                        LayerType::Tiles | LayerType::IntGrid | LayerType::Autolayer,
                    ) if ids.contains(&layer_json.identifier) => {
                        spawn_tile_layer(&mut entity_commands, layer);
                    }
                    (
                        LoadLayers::TileLayers | LoadLayers::All,
                        LayerType::Tiles | LayerType::IntGrid | LayerType::Autolayer,
                    ) => {
                        spawn_tile_layer(&mut entity_commands, layer);
                    }

                    // Entity layer variants
                    (LoadLayers::ByIdentifiers(ids), LayerType::Entities)
                        if ids.contains(&layer_json.identifier) =>
                    {
                        spawn_entity_layer(&mut entity_commands, layer);
                    }
                    (LoadLayers::ByIids(ids), LayerType::Entities)
                        if ids.contains(&layer_json.identifier) =>
                    {
                        spawn_entity_layer(&mut entity_commands, layer);
                    }
                    (LoadLayers::EntityLayers | LoadLayers::All, LayerType::Entities) => {
                        spawn_entity_layer(&mut entity_commands, layer);
                    }

                    // ignore me!
                    _ => (),
                };
            }
        }

        entity_commands
            .insert((Name::from(level_component.identifier()), level_component))
            .remove::<LevelBundleLoadStub>();
    }

    Ok(())
}
