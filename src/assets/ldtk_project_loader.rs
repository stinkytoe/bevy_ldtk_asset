use bevy::asset::io::Reader;
use bevy::asset::AssetLoader;
use bevy::asset::AsyncReadExt;
use bevy::asset::LoadContext;
use bevy::prelude::*;
use bevy::utils::thiserror;
use std::path::PathBuf;
use thiserror::Error;

use crate::assets::ldtk_level::LdtkLevel;
use crate::assets::ldtk_project::LdtkProject;
use crate::ldtk_json;

#[derive(Debug, Error)]
pub(crate) enum LdtkProjectLoaderError {
	#[error("IO error when reading asset: {0}")]
	Io(#[from] std::io::Error),
	#[error("Unable to parse JSON! {0}")]
	UnableToParse(#[from] serde_json::Error),
	// #[error("Path Error: {0}")]
	// PathError(#[from] LdtkPathError),
	#[error("Couldn't load child LDTk level! {0}")]
	UnableToLoadExternalChild(#[from] bevy::asset::LoadDirectError),
	#[error("Couldn't get parent of asset path! {0}")]
	UnableToGetParent(PathBuf),
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
						load_context.add_loaded_labeled_asset(
							level_json.identifier.clone(),
							level_asset.into(),
						);
					};
				}
			} else {
				value.levels.iter().for_each(|level| {
					load_context.add_labeled_asset(
						level.identifier.clone(),
						LdtkLevel {
							value: level.clone(),
						},
					);
				});
			}

			debug!(
				"LDtk root project file: {} loaded!",
				load_context.path().to_str().unwrap_or_default()
			);

			Ok(LdtkProject { value })
		})
	}

	fn extensions(&self) -> &[&str] {
		&["ldtk"]
	}
}
