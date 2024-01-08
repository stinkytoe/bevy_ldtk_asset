use crate::ldtk::level_asset::LevelAsset;
use bevy::asset::{io::Reader, AssetLoader, LoadContext};
use bevy::prelude::*;
use bevy::utils::thiserror;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum LdtkLevelLoaderError {
    #[error("LDtk external level file loading not yet implemented!")]
    NotYetImplemented,
    #[error("IO error when reading asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Unable to parse JSON! {0}")]
    UnableToParse(#[from] serde_json::Error),
    // #[error("Couldn't get parent of asset path! {0}")]
    // UnableToGetParent(PathBuf),
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) struct LdtkLevelLoaderSettings;

#[derive(Default)]
pub(crate) struct LdtkLevelLoader;

impl AssetLoader for LdtkLevelLoader {
    type Asset = LevelAsset;
    type Settings = LdtkLevelLoaderSettings;
    type Error = LdtkLevelLoaderError;

    fn load<'a>(
        &'a self,
        _reader: &'a mut Reader,
        _settings: &'a LdtkLevelLoaderSettings,
        _load_context: &'a mut LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            // debug!(
            //     "Loading LDtk level file: {}",
            //     load_context.path().to_str().unwrap_or_default()
            // );
            //
            // let value: ldtk_json::Level = {
            //     let mut bytes = Vec::new();
            //     reader.read_to_end(&mut bytes).await?;
            //     serde_json::from_slice(&bytes)?
            // };
            //
            // let load_context_path_buf = load_context.path().to_path_buf();
            //
            // let ldtk_project_directory = if let Some(parent) = load_context_path_buf.parent() {
            //     PathBuf::from(parent)
            // } else {
            //     return Err(LdtkLevelLoaderError::UnableToGetParent(
            //         load_context_path_buf,
            //     ));
            // };
            //
            // let ldtk_extras_directory = if let Some(file_stem) = load_context_path_buf.file_stem() {
            //     ldtk_project_directory.join(file_stem)
            // } else {
            //     return Err(LdtkLevelLoaderError::UnableToGetParent(
            //         load_context_path_buf,
            //     ));
            // };
            //
            // debug!(
            //     "LDtk level file: {} loaded!",
            //     load_context.path().to_str().unwrap_or_default()
            // );
            //
            // Ok(LdtkLevel::new(
            //     value,
            //     ldtk_project_directory,
            //     ldtk_extras_directory,
            //     load_context.load(""),
            //     // Vec::new(),
            //     load_context, // bg_image,
            // ))
            Err(LdtkLevelLoaderError::NotYetImplemented)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtkl"]
    }
}
