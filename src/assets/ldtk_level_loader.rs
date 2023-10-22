use super::ldtk_level::LdtkLevel;
use bevy::asset::AsyncReadExt;
use bevy::asset::{io::Reader, AssetLoader, LoadContext};
use bevy::utils::thiserror;
use thiserror::Error;
// use super::ldtk_level::LdtkLevel;

#[derive(Debug, Error)]
pub(crate) enum LdtkRootLoaderError {
    /// An [IO](std::io) Error
    #[error("Could load raw asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Unable to parse given color string! {0}")]
    UnableToParse(#[from] serde_json::Error),
}

#[derive(Default)]
pub(crate) struct LdtkLevelLoader;

impl AssetLoader for LdtkLevelLoader {
    type Asset = LdtkLevel;
    type Settings = ();
    type Error = LdtkRootLoaderError;

    fn load<'a>(
        // &'a self,
        // bytes: &'a [u8],
        // load_context: &'a mut bevy::asset::LoadContext,
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        _load_context: &'a mut LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            // debug!(
            //     "Loading LDtk level file: {}",
            //     load_context.path().to_str().unwrap_or_default()
            // );
            //
            // let value: ldtk_json::Level = serde_json::from_slice(bytes)?;
            //
            // // load_context.set_default_asset(LoadedAsset::new(LdtkLevel {
            // //     _level: Level::new(&value, load_context),
            // // }));
            //
            // load_context.set_default_asset(LoadedAsset::new(LdtkLevel { _level: value }));
            //
            // debug!(
            //     "Loading LDtk level file: {} success!",
            //     load_context.path().to_str().unwrap_or_default()
            // );
            //
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            Ok(LdtkLevel {
                _level: serde_json::from_slice(&bytes)?,
            })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtkl"]
    }
}
