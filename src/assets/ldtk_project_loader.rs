use bevy::asset::io::Reader;
use bevy::asset::AssetLoader;
use bevy::asset::AsyncReadExt;
use bevy::asset::LoadContext;
use bevy::prelude::*;
use bevy::utils::thiserror;
use std::collections::HashMap;
use thiserror::Error;

use crate::assets::ldtk_level::LdtkLevel;
use crate::assets::ldtk_project::LdtkProject;
use crate::ldtk_json;
use crate::util::get_bevy_path_from_ldtk_path;
use crate::util::LdtkPathError;

#[derive(Debug, Error)]
pub(crate) enum LdtkProjectLoaderError {
	#[error("IO error when reading asset: {0}")]
	Io(#[from] std::io::Error),
	#[error("Unable to parse JSON! {0}")]
	UnableToParse(#[from] serde_json::Error),
	#[error("Path Error: {0}")]
	PathError(#[from] LdtkPathError),
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

			let context_path_buf = load_context.path().to_path_buf();

			// First return an iterator of tuples which contain
			// the world name and the list of levels in that world.
			//
			// For single world projects, we use only the level name as
			// the label. So you would load like:
			//
			// asset_server.get("project.ldtk#Level_0");
			//
			// For multi world projects, we cat the world name, and a slash '/'
			// and the level name. In this case you would load like:
			//
			// asset_server.get("project.ldtk#World/Level_0");
			let level_handle_map = if value.worlds.is_empty() {
				vec![("".to_owned(), value.levels.iter())]
			} else {
				value
					.worlds
					.iter()
					.map(|world| (world.identifier.clone() + "/", world.levels.iter()))
					.collect()
			}
			.iter()
			// here we take each touple from before, and create a list
			// of tuples with their json level representation
			.flat_map(|(world_id, levels)| {
				levels.clone().map(|level| {
					(
						world_id.to_owned() + &level.identifier.clone(),
						level.clone(),
					)
				})
			})
			// now each tuple is reconstructed with a handle to their
			// asset in the second part. Its label is used so that we can
			// find it using bevy's labeled asset loading syntax
			.map(|(label, level)| {
				Ok((
					label.to_owned(),
					if let Some(level_path) = level.external_rel_path {
						load_context.load(
							get_bevy_path_from_ldtk_path(&context_path_buf, &level_path).unwrap(),
						)
					} else {
						load_context.add_labeled_asset(
							label.to_owned(),
							LdtkLevel {
								value: level.to_owned(),
							},
						)
					},
				))
			})
			.collect::<Result<HashMap<String, Handle<LdtkLevel>>, LdtkPathError>>()?;

			// debug!("World/Level pairs: \n{level_handle_map:#?}");

			debug!(
				"LDtk root project file: {} loaded!",
				load_context.path().to_str().unwrap_or_default()
			);

			Ok(LdtkProject {
				value,
				level_handle_map,
			})
		})
	}

	fn extensions(&self) -> &[&str] {
		&["ldtk"]
	}
}
