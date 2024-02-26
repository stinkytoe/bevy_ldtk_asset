use std::path::{Path, PathBuf};

use bevy::asset::{AssetLoader, AsyncReadExt, LoadContext, ReadAssetBytesError};
use bevy::prelude::*;
use bevy::tasks::futures_lite;
use bevy::utils::{thiserror, HashMap};
use futures_lite::stream::{self, StreamExt};
use thiserror::Error;

use crate::assets::level::LevelAsset;
use crate::ldtk;
use crate::prelude::WorldAsset;
use crate::util::ldtk_path_to_asset_path;

use super::project::ProjectAsset;

const SINGLE_WORLD_NAME: &str = "World";

#[derive(Debug, Error)]
pub(crate) enum ProjectAssetLoaderError {
    #[error("IO error! {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON Parse error! {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("Read Asset Bytes error! {0}")]
    ReadAssetBytes(#[from] ReadAssetBytesError),
    #[error("Could not get project directory? {0}")]
    BadProjectDirectory(PathBuf),
}

#[derive(Default)]
pub(crate) struct ProjectAssetLoader;

impl AssetLoader for ProjectAssetLoader {
    type Asset = ProjectAsset;

    type Settings = ();

    type Error = ProjectAssetLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let asset_path = load_context.path().to_path_buf();

            debug!("Loading LDtk project file: {asset_path:?}");

            let value: ldtk::LdtkJson = {
                let mut bytes = Vec::new();
                reader.read_to_end(&mut bytes).await?;
                serde_json::from_slice(&bytes)?
            };

            let base_directory = asset_path
                .parent()
                .ok_or(ProjectAssetLoaderError::BadProjectDirectory(
                    asset_path.clone(),
                ))?
                .to_path_buf();

            let exports_directory = base_directory.join(asset_path.file_stem().ok_or(
                ProjectAssetLoaderError::BadProjectDirectory(asset_path.clone()),
            )?);

            Ok(ProjectAsset {
                tilesets: build_tilesets(&value, load_context, &base_directory),
                worlds: build_worlds(&value, load_context),
                levels: build_levels(&value, load_context, &base_directory).await?,
                backgrounds: Vec::default(),
                asset_path,
                base_directory,
                exports_directory,
            })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtk"]
    }
}

fn build_tilesets(
    value: &ldtk::LdtkJson,
    load_context: &mut LoadContext<'_>,
    base_directory: &Path,
) -> Vec<Handle<Image>> {
    value
        .defs
        .tilesets
        .iter()
        .filter_map(|tileset_definition| tileset_definition.rel_path.as_ref())
        .map(|rel_path| load_context.load(base_directory.join(rel_path)))
        .collect()
}

fn build_worlds(
    value: &ldtk::LdtkJson,
    load_context: &mut LoadContext,
) -> HashMap<String, Handle<WorldAsset>> {
    if value.worlds.is_empty() {
        [(
            SINGLE_WORLD_NAME.to_string(),
            load_context.add_labeled_asset(SINGLE_WORLD_NAME.to_string(), value.into()),
        )]
        .into()
    } else {
        value
            .worlds
            .iter()
            // .cloned()
            .map(|world| {
                let world: WorldAsset = world.into();
                (
                    world.identifier().clone(),
                    load_context.add_labeled_asset(world.identifier().clone(), world),
                )
            })
            .collect()
    }
}

async fn build_levels(
    value: &ldtk::LdtkJson,
    load_context: &mut LoadContext<'_>,
    base_directory: &Path,
) -> Result<HashMap<String, Handle<LevelAsset>>, ProjectAssetLoaderError> {
    let all_levels = if value.worlds.is_empty() {
        value
            .levels
            .iter()
            .map(|level| (SINGLE_WORLD_NAME.to_string(), level))
            .collect::<Vec<_>>()
    } else {
        value
            .worlds
            .iter()
            .map(|world| (world.identifier.clone(), world))
            .flat_map(|(identifier, world)| {
                world
                    .levels
                    .iter()
                    .map(move |level| (identifier.clone(), level))
            })
            .collect::<Vec<_>>()
    };

    let mut ret = HashMap::default();
    let mut levels_stream = stream::iter(all_levels);

    while let Some((identifier, level)) = levels_stream.next().await {
        let level = if value.external_levels {
            let bytes = load_context
                .read_asset_bytes(ldtk_path_to_asset_path(
                    base_directory,
                    Path::new(
                        &level
                            .external_rel_path
                            .as_ref()
                            .expect("external_rel_path is 'None' when external levels is true?")
                            .clone(),
                    ),
                ))
                .await?;
            LevelAsset::new(&serde_json::from_slice(&bytes)?)
        } else {
            LevelAsset::new(level)
        };

        ret.insert(
            identifier.clone(),
            load_context.add_labeled_asset(identifier, level),
        );
    }

    Ok(ret)

    // Ok(if value.worlds.is_empty() {
    //     value
    //         .levels
    //         .iter()
    //         .map(|level| (SINGLE_WORLD_NAME.to_string(), level))
    //         .collect::<Vec<_>>()
    // } else {
    //     value
    //         .worlds
    //         .iter()
    //         .map(|world| (world.identifier.clone(), world))
    //         .flat_map(|(identifier, world)| {
    //             world
    //                 .levels
    //                 .iter()
    //                 .map(move |level| (identifier.clone(), level))
    //         })
    //         .collect::<Vec<_>>()
    // }
    // .iter()
    // .map(|(identifier, level)| {
    //     // let x = load_context.read_asset_bytes("").await?;
    //     // let x = stream::iter(worlds.clone())
    //     //     .map(|(a, b)| b)
    //     //     .collect::<Vec<_>>()
    //     //     .await;
    //     (
    //         identifier,
    //         if value.external_levels {
    //             // let bytes = load_context.read_asset_bytes("").await;
    //             unimplemented!()
    //         } else {
    //             LevelAsset::new(level)
    //         },
    //     )
    // })
    // .map(|(identifier, level)| {
    //     (
    //         level.identifier().clone(),
    //         load_context
    //             .add_labeled_asset(identifier.to_owned() + "/" + &level.identifier(), level),
    //     )
    // })
    // .collect())
}
