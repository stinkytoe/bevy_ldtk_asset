use crate::assets::ldtk_level::LdtkLevel;
use crate::ldtk_json;
use bevy::asset::AsyncReadExt;
use bevy::asset::{io::Reader, AssetLoader, LoadContext};
use bevy::prelude::*;
use bevy::utils::thiserror;
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum LdtkLevelLoaderError {
    #[error("IO error when reading asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Unable to parse JSON! {0}")]
    UnableToParse(#[from] serde_json::Error),
}

#[derive(Default)]
pub(crate) struct LdtkLevelLoader;

impl AssetLoader for LdtkLevelLoader {
    type Asset = LdtkLevel;
    type Settings = ();
    type Error = LdtkLevelLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        load_context: &'a mut LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            debug!(
                "Loading LDtk level file: {}",
                load_context.path().to_str().unwrap_or_default()
            );

            let value: ldtk_json::Level = {
                let mut bytes = Vec::new();
                reader.read_to_end(&mut bytes).await?;
                serde_json::from_slice(&bytes)?
            };

            debug!(
                "LDtk level file: {} loaded!",
                load_context.path().to_str().unwrap_or_default()
            );

            Ok(LdtkLevel { value })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtkl"]
    }
}
