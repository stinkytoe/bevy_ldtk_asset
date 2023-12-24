use crate::assets::ldtk_level::LdtkLevel;
use crate::assets::ldtk_project::LdtkProject;
use crate::ldtk_json;
use bevy::asset::io::Reader;
use bevy::asset::AssetLoader;
use bevy::asset::AsyncReadExt;
use bevy::asset::LoadContext;
use bevy::prelude::*;
use bevy::utils::thiserror;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum LdtkProjectLoaderError {
    #[error("IO error when reading asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Unable to parse JSON! {0}")]
    UnableToParse(#[from] serde_json::Error),
    #[error("Couldn't load child LDTk level! {0}")]
    UnableToLoadExternalChild(#[from] bevy::asset::LoadDirectError),
    #[error("Couldn't get parent of asset path! {0}")]
    UnableToGetParent(PathBuf),
    #[error("Couldn't get file stem of asset path! {0}")]
    UnableToGetFileStem(PathBuf),
}

#[derive(Default)]
pub(crate) struct LdtkProjectLoader;

impl AssetLoader for LdtkProjectLoader {
    type Asset = LdtkProject;
    type Settings = ();
    type Error = LdtkProjectLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        load_context: &'a mut LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            debug!(
                "Loading LDtk root project file: {}",
                load_context.path().to_str().unwrap_or_default()
            );

            let value: ldtk_json::LdtkJson = {
                let mut bytes = Vec::new();
                reader.read_to_end(&mut bytes).await?;
                serde_json::from_slice(&bytes)?
            };

            let load_context_path_buf = load_context.path().to_path_buf();
            let load_context_directory = if let Some(parent) = load_context_path_buf.parent() {
                PathBuf::from(parent)
            } else {
                return Err(LdtkProjectLoaderError::UnableToGetParent(
                    load_context_path_buf,
                ));
            };

            let mut level_handles: Vec<Handle<LdtkLevel>> = Vec::new();

            // loading levels
            if value.external_levels {
                for (level_asset_path, level_json) in value.levels.iter().filter_map(|level_json| {
                    level_json.external_rel_path.as_ref().map(|level_path| {
                        (load_context_directory.clone().join(level_path), level_json)
                    })
                }) {
                    if let Some(level_asset) = load_context
                        .load_direct(level_asset_path)
                        .await?
                        .take::<LdtkLevel>()
                    {
                        level_handles.push(load_context.add_loaded_labeled_asset(
                            level_json.identifier.clone(),
                            level_asset.into(),
                        ));
                    };
                }
            } else {
                for level in value.levels.iter() {
                    if let Some(prefix) = load_context_path_buf.file_stem() {
                        let images: Vec<Handle<Image>> = Vec::new();

                        let new_level = LdtkLevel::new(
                            level.clone(),
                            load_context_directory.join(prefix),
                            images,
                            level.bg_rel_path.as_ref().map(|bg_rel_path| {
                                load_context.load(load_context_directory.join(bg_rel_path))
                            }),
                        );

                        level_handles.push(
                            load_context.add_labeled_asset(level.identifier.clone(), new_level),
                        );
                    } else {
                        return Err(LdtkProjectLoaderError::UnableToGetFileStem(
                            load_context_path_buf,
                        ));
                    };
                }
            }

            // value.defs.tilesets.iter().for_each(|tileset| {
            //     if let Some(rel_path) = &tileset.rel_path {
            //         load_context.load::<Image>(load_context_directory.join(rel_path));
            //     };
            // });

            debug!(
                "LDtk root project file: {} loaded!",
                load_context.path().to_str().unwrap_or_default()
            );

            Ok(LdtkProject {
                value,
                levels: level_handles,
            })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtk"]
    }
}
