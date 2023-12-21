use std::path::PathBuf;

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
	#[error("Couldn't get parent of asset path! {0}")]
	UnableToGetParent(PathBuf),
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

			let load_context_path_buf = load_context.path().to_path_buf();
			let load_context_directory = if let Some(parent) = load_context_path_buf.parent() {
				PathBuf::from(parent)
			} else {
				return Err(LdtkLevelLoaderError::UnableToGetParent(
					load_context_path_buf,
				));
			};

			debug!(
				"LDtk level file: {} loaded!",
				load_context.path().to_str().unwrap_or_default()
			);

			Ok(LdtkLevel::new(value, load_context_directory))
		})
	}

	fn extensions(&self) -> &[&str] {
		&["ldtkl"]
	}
}
