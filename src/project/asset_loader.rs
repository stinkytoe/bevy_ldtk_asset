use bevy::asset::AssetLoader;
use bevy::asset::AsyncReadExt;
use bevy::prelude::*;
use bevy::utils::HashMap;
use std::path::PathBuf;
use thiserror::Error;

use crate::ldtk;
use crate::project::ProjectAsset;
use crate::project::ProjectSettings;
use crate::util::bevy_color_from_ldtk;
use crate::util::ColorParseError;
use crate::world::WorldAsset;

#[derive(Debug, Error)]
pub(crate) enum ProjectAssetLoaderError {
    #[error("IO error! {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON Parse error! {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("Could not get project directory? {0}")]
    BadProjectDirectory(PathBuf),
    #[error("ColorParseError! {0}")]
    ColorParseError(#[from] ColorParseError),
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

            let world_tuples: Box<dyn Iterator<Item = (WorldAsset, String, &Vec<ldtk::Level>)>> =
                if value.worlds.is_empty() {
                    Box::new(
                        [(
                            WorldAsset {
                                project: project_handle.clone(),
                                iid: value.iid.clone(),
                            },
                            "World".to_owned(),
                            &value.levels,
                        )]
                        .into_iter(),
                    )
                } else {
                    Box::new(value.worlds.iter().map(|world| {
                        (
                            WorldAsset {
                                project: project_handle.clone(),
                                iid: world.iid.clone(),
                            },
                            world.identifier.clone(),
                            &world.levels,
                        )
                    }))
                };

            for (world_asset, world_identifier, levels) in world_tuples {
                let iid = world_asset.iid.clone();
                let world_handle =
                    load_context.add_loaded_labeled_asset(world_identifier, world_asset.into());
                world_assets.insert(iid, world_handle);
            }

            // let levels: Box<dyn Iterator<Item = &ldtk::Level>> = if value.worlds.is_empty() {
            //     Box::new(value.levels.iter())
            // } else {
            //     Box::new(value.worlds.iter().flat_map(|world| world.levels.iter()))
            // };
            //
            // for level in levels {
            //     let level_asset = LevelAsset {
            //         project: project_handle,
            //         iid: level.iid.clone(),
            //     };
            //
            //     let tag = format!("{}/{}", world.identifier, level.identifier);
            //
            //     let level_handle = load_context.add_loaded_labeled_asset(tag, level_asset.into());
            //     level_assets.insert(level.iid, level_handle);
            // }

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
