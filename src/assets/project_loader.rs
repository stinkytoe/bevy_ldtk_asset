use std::path::PathBuf;

use bevy::asset::{AssetLoader, AsyncReadExt};
use bevy::prelude::*;
use bevy::utils::{thiserror, HashMap};
use thiserror::Error;

use crate::ldtk::{self};

use super::project::ProjectAsset;

#[derive(Debug, Error)]
pub(crate) enum ProjectLoaderError {
    #[error("IO Error! {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON Parse error! {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("Could not get project directory? {0}")]
    BadProjectDirectory(PathBuf),
}

#[derive(Default)]
pub(crate) struct ProjectLoader;

impl AssetLoader for ProjectLoader {
    type Asset = ProjectAsset;

    type Settings = ();

    type Error = ProjectLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let file_name = load_context.path().to_path_buf();

            debug!("Loading LDtk project file: {file_name:?}");

            let value: ldtk::LdtkJson = {
                let mut bytes = Vec::new();
                reader.read_to_end(&mut bytes).await?;
                serde_json::from_slice(&bytes)?
            };

            let asset_path = load_context.path().to_path_buf();

            let base_directory = asset_path
                .parent()
                .ok_or(ProjectLoaderError::BadProjectDirectory(asset_path.clone()))?
                .to_path_buf();

            let exports_directory = base_directory.join(
                asset_path
                    .file_stem()
                    .ok_or(ProjectLoaderError::BadProjectDirectory(asset_path.clone()))?,
            );

            value.defs.tilesets.iter().for_each(|tileset_definition| {
                if let Some(tileset_path) = &tileset_definition.rel_path {
                    // load_context.load_untyped(asset_path.join(tileset_path));
                    info!("{:?}", base_directory.join(tileset_path));
                }
            });

            // let worlds = if value.worlds.is_empty() {
            //     let world: LdtkWorld = value.into();
            //     [(
            //         world.identifier().clone(),
            //         load_context.add_labeled_asset(world.identifier().clone(), world),
            //     )]
            //     .into()
            // } else {
            //     value
            //         .worlds
            //         .iter()
            //         .cloned()
            //         .map(|world| {
            //             let world: LdtkWorld = world.into();
            //             (
            //                 world.identifier().clone(),
            //                 load_context.add_labeled_asset(world.identifier().clone(), world),
            //             )
            //         })
            //         .collect()
            // };

            let worlds = HashMap::default();

            debug!("LDtk project file {file_name:?} loaded!");

            Ok(ProjectAsset {
                asset_path,
                base_directory,
                exports_directory,
                worlds,
            })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtk"]
    }
}
