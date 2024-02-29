use std::path::{Path, PathBuf};

use bevy::asset::{AssetLoader, AsyncReadExt, LoadContext, ReadAssetBytesError};
use bevy::prelude::*;
use bevy::tasks::futures_lite;
use bevy::utils::{thiserror, HashMap};
use futures_either::Either;
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
                tilesets: build_tilesets(&value, load_context, &base_directory).await,
                backgrounds: build_backgrounds(&value, load_context, &base_directory).await,
                levels: build_levels(&value, load_context, &base_directory).await?,
                worlds: build_worlds(&value, load_context).await,
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

async fn build_tilesets(
    value: &ldtk::LdtkJson,
    load_context: &mut LoadContext<'_>,
    base_directory: &Path,
) -> HashMap<String, Handle<Image>> {
    stream::iter(value.defs.tilesets.iter())
        .filter_map(|tileset_definition| tileset_definition.rel_path.as_ref())
        // .map(|rel_path| load_context.load(base_directory.join(rel_path)))
        .map(|ldtk_path| {
            (
                ldtk_path.clone(),
                load_context.load(ldtk_path_to_asset_path(
                    base_directory,
                    &PathBuf::from(ldtk_path),
                )),
            )
        })
        .collect()
        .await
}

async fn build_backgrounds<'a>(
    value: &'a ldtk::LdtkJson,
    load_context: &'a mut LoadContext<'_>,
    base_directory: &'a Path,
) -> HashMap<String, Handle<Image>> {
    stream::iter(if value.worlds.is_empty() {
        Either::Left(value.levels.iter())
    } else {
        Either::Right(value.worlds.iter().flat_map(|world| world.levels.iter()))
    })
    .filter_map(|level| level.bg_rel_path.as_ref())
    .map(|ldtk_path| {
        (
            ldtk_path.clone(),
            load_context.load(ldtk_path_to_asset_path(
                base_directory,
                &PathBuf::from(ldtk_path),
            )),
        )
    })
    .collect()
    .await
}

async fn build_worlds(
    value: &ldtk::LdtkJson,
    load_context: &mut LoadContext<'_>,
) -> HashMap<String, Handle<WorldAsset>> {
    if value.worlds.is_empty() {
        [(
            SINGLE_WORLD_NAME.to_string(),
            load_context.add_labeled_asset(
                SINGLE_WORLD_NAME.to_string(),
                WorldAsset::new_from_ldtk_json(value),
            ),
        )]
        .into()
    } else {
        stream::iter(value.worlds.iter())
            .map(|world| {
                let world: WorldAsset = world.into();
                (
                    world.identifier().clone(),
                    load_context.add_labeled_asset(world.identifier().clone(), world),
                )
            })
            .collect()
            .await
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
    let mut stream = stream::iter(all_levels);

    // for (identifier, level) in all_levels {
    while let Some((identifier, level)) = stream.next().await {
        let level = if value.external_levels {
            let ldtk_path = ldtk_path_to_asset_path(
                base_directory,
                Path::new(
                    &level
                        .external_rel_path
                        .as_ref()
                        .expect("external_rel_path is 'None' when external levels is true?")
                        .clone(),
                ),
            );
            // debug!("{ldtk_path:?}");
            let bytes = load_context.read_asset_bytes(ldtk_path).await;
            // debug!("{bytes:?}");
            let parsed = serde_json::from_slice(&bytes?)?;
            // debug!("{parsed:?}");
            LevelAsset::new(&parsed)
        } else {
            LevelAsset::new(level)
        };

        let k = identifier.clone() + "/" + level.identifier();
        // let asset =
        //     load_context.add_labeled_asset(identifier.clone() + "/" + level.identifier(), level);

        let labeled = load_context.begin_labeled_asset();
        let loaded_asset = labeled.finish(level, None);
        //
        let v = load_context.add_loaded_labeled_asset(k.clone(), loaded_asset);

        ret.insert(k, v);
        // let mut handles = Vec::new();
        // for i in 0..2 {
        //     let mut labeled = load_context.begin_labeled_asset();
        //     handles.push(std::thread::spawn(move || {
        //         (i.to_string(), labeled.finish(Image::default(), None))
        //     }));
        // }
        // for handle in handles {
        //     let (label, loaded_asset) = handle.join().unwrap();
        //     load_context.add_loaded_labeled_asset(label, loaded_asset);
        // }

        // ret.insert(
        //     identifier.clone(),
        //     load_context.add_loaded_labeled_asset(
        //         identifier.to_string() + "/" + level.identifier(),
        //         level.into(),
        //     ),
        // );
    }

    Ok(ret)
}
