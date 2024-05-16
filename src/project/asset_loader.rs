use bevy::asset::AssetLoader;
use bevy::asset::AsyncReadExt;
use bevy::asset::ReadAssetBytesError;
use bevy::prelude::*;
use bevy::utils::HashMap;
use std::path::Path;
use std::path::PathBuf;
use thiserror::Error;

use crate::layer::LayerTypeError;
use crate::ldtk;
use crate::level::LevelAsset;
use crate::level::NewLevelAssetError;
use crate::prelude::LayerAsset;
use crate::project::ProjectAsset;
use crate::project::ProjectSettings;
use crate::util::bevy_color_from_ldtk;
use crate::util::ldtk_path_to_asset_path;
use crate::util::ColorParseError;
use crate::world::NewWorldAssetError;
use crate::world::WorldAsset;

#[derive(Debug, Error)]
pub(crate) enum ProjectAssetLoaderError {
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("{0}")]
    ColorParseError(#[from] ColorParseError),
    #[error("{0}")]
    NewWorldAssetError(#[from] NewWorldAssetError),
    #[error("{0}")]
    NewLevelAssetError(#[from] NewLevelAssetError),
    #[error("{0}")]
    ReadAssetBytesError(#[from] ReadAssetBytesError),
    #[error("{0}")]
    LayerTypeError(#[from] LayerTypeError),
    #[error("Could not get project directory? {0}")]
    BadProjectDirectory(PathBuf),
    #[error("externalRelPath is None when external levels is true?")]
    ExternalRelPathIsNone,
}

#[derive(Debug, Default)]
pub(crate) struct ProjectAssetLoader;

impl AssetLoader for ProjectAssetLoader {
    type Asset = ProjectAsset;

    type Settings = ProjectSettings;

    type Error = ProjectAssetLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let asset_path = load_context.path().to_path_buf();

            info!("Loading LDtk project file: {asset_path:?}");

            let base_directory = asset_path
                .parent()
                .ok_or(ProjectAssetLoaderError::BadProjectDirectory(
                    asset_path.clone(),
                ))?
                .to_path_buf();

            let project_handle: Handle<ProjectAsset> =
                load_context.load(load_context.path().to_path_buf());

            let value: ldtk::LdtkJson = {
                let mut bytes = Vec::new();
                reader.read_to_end(&mut bytes).await?;
                serde_json::from_slice(&bytes)?
            };

            let mut world_assets = HashMap::default();
            let mut level_assets = HashMap::default();
            let mut layer_assets = HashMap::default();
            let mut entity_assets = HashMap::default();

            let mut tileset_assets = HashMap::default();
            let mut background_assets = HashMap::default();

            let world_tuples: Vec<_> = if value.worlds.is_empty() {
                vec![(
                    WorldAsset::new_from_project(&value, project_handle.clone())?,
                    "World".to_owned(),
                    &value.levels,
                )]
            } else {
                value
                    .worlds
                    .iter()
                    .map(|world| {
                        Ok((
                            WorldAsset::new_from_world(world, project_handle.clone())?,
                            world.identifier.clone(),
                            &world.levels,
                        ))
                    })
                    .collect::<Result<Vec<_>, ProjectAssetLoaderError>>()?
            };

            for (world_asset, world_identifier, levels) in world_tuples {
                let iid = world_asset.iid.clone();
                let world_handle = load_context
                    .add_loaded_labeled_asset(world_identifier.clone(), world_asset.into());

                debug!("Added new WorldAsset!");
                debug!("identifier: {world_identifier}");
                debug!("iid: {iid}");

                world_assets.insert(iid, world_handle);

                for level in levels.iter() {
                    let iid = level.iid.clone();
                    let level_asset = LevelAsset::new(level, project_handle.clone())?;
                    let level_label = format!("{}/{}", world_identifier, level.identifier);
                    let level_handle =
                        load_context.add_loaded_labeled_asset(level_label, level_asset.into());

                    debug!("Added new LevelAsset!");
                    debug!("identifier: {}", level.identifier);
                    debug!("iid: {iid}");

                    level_assets.insert(iid, level_handle);

                    let Some(layer_instances) = ({
                        if !value.external_levels {
                            level.layer_instances.clone()
                        } else {
                            let level_path = level
                                .external_rel_path
                                .as_ref()
                                .ok_or(ProjectAssetLoaderError::ExternalRelPathIsNone)?;
                            let level_path = Path::new(level_path);
                            let level_path = ldtk_path_to_asset_path(&base_directory, level_path);
                            let bytes = load_context.read_asset_bytes(level_path).await?;
                            let level_json: ldtk::Level = serde_json::from_slice(&bytes)?;
                            level_json.layer_instances
                        }
                    }) else {
                        break;
                    };

                    for layer_instance in layer_instances.iter().rev() {
                        let iid = layer_instance.iid.clone();
                        let layer_asset = LayerAsset::new(layer_instance, project_handle.clone())?;
                        let layer_label = format!(
                            "{}/{}/{}",
                            world_identifier, level.identifier, layer_instance.identifier
                        );
                        let layer_handle =
                            load_context.add_loaded_labeled_asset(layer_label, layer_asset.into());

                        debug!("Added new LayerAsset!");
                        debug!("identifier: {}", layer_instance.identifier);
                        debug!("iid: {iid}");

                        layer_assets.insert(iid, layer_handle);
                    }
                }
            }

            Ok(ProjectAsset {
                bg_color: bevy_color_from_ldtk(&value.bg_color)?,
                external_levels: value.external_levels,
                iid: value.iid.clone(),
                json_version: value.json_version.clone(),
                world_assets,
                level_assets,
                layer_assets,
                entity_assets,
                tileset_assets,
                background_assets,
                settings: settings.clone(),
            })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtk"]
    }
}
