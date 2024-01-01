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
    // #[error("Couldn't get file stem of asset path! {0}")]
    // UnableToGetFileStem(PathBuf),
    #[error("External level files unsupported at this time!")]
    UnsupportedExternalLevelFiles,
    #[error("Multi World unsupported at this time!")]
    UnsupportedMultiWorld,
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

            let ldtk_project_directory = if let Some(parent) = load_context_path_buf.parent() {
                PathBuf::from(parent)
            } else {
                return Err(LdtkProjectLoaderError::UnableToGetParent(
                    load_context_path_buf,
                ));
            };

            let ldtk_extras_directory = if let Some(file_stem) = load_context_path_buf.file_stem() {
                ldtk_project_directory.join(file_stem)
            } else {
                return Err(LdtkProjectLoaderError::UnableToGetParent(
                    load_context_path_buf,
                ));
            };

            if !value.worlds.is_empty() {
                return Err(LdtkProjectLoaderError::UnsupportedMultiWorld);
            }

            let self_handle: Handle<LdtkProject> = load_context.load(load_context_path_buf.clone());

            if value.external_levels {
                return Err(LdtkProjectLoaderError::UnsupportedExternalLevelFiles);
            } else {
                value.levels.iter().for_each(|level| {
                    let new_level = LdtkLevel::new(
                        level.clone(),
                        ldtk_project_directory.clone(),
                        ldtk_extras_directory.clone(),
                        &value,
                        self_handle.clone(),
                        load_context,
                    );
                    load_context.add_labeled_asset(level.identifier.clone(), new_level);
                });
            }

            debug!(
                "LDtk root project file: {} loaded!",
                load_context.path().to_str().unwrap_or_default()
            );

            Ok(LdtkProject {
                value,
                // levels: level_handles,
            })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtk"]
    }
}
