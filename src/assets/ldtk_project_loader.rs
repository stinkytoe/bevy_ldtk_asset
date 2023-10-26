use crate::assets::ldtk_level::LdtkLevel;
use crate::assets::ldtk_project::LdtkProject;
use crate::assets::structs::level::{Level, LevelError};
use crate::assets::structs::world::{World, WorldError};
// use crate::assets::util::ldtk_file_to_asset_path;
use crate::ldtk_json;
use crate::util::{ldtk_project_path_join, ColorParseError};
use bevy::asset::{io::Reader, AssetLoader, LoadContext};
use bevy::asset::{AssetPath, AsyncReadExt};
use bevy::prelude::*;
use bevy::utils::thiserror;
use std::collections::HashMap;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum LdtkRootLoaderError {
    // #[error("Provided color string not seven characters! {0}")]
    // BadStringLength(&'a str),
    // #[error("Unable to parse given color string! {0}")]
    // UnableToParse(&'a str),
    #[error("Failed to parse color: {0}")]
    ColorParse(#[from] ColorParseError),
    #[error("Could load raw asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Unable to parse given color string! {0}")]
    UnableToParse(#[from] serde_json::Error),
    #[error("Failed to construct World: {0}")]
    BadWorld(#[from] WorldError),
    #[error("Failed to construct Level: {0}")]
    BadLevel(#[from] LevelError),
    #[error("Failed to construct external level")]
    BadExternalLevel,
    #[error("Bevy failed to load asset: {0}")]
    BevyLoadDirectError(#[from] bevy::asset::LoadDirectError),
    #[error("Unable to find root folder of project: {0}")]
    BadAssetPath(PathBuf),
    #[error("Unable to parse path into string: {0}")]
    PathUnparsable(PathBuf),
    #[error("Unable to parse path into string: {0}")]
    PathJoinUnparsable(String),
}

#[derive(Default)]
pub(crate) struct LdtkRootLoader;

impl AssetLoader for LdtkRootLoader {
    type Asset = LdtkProject;
    type Settings = ();
    type Error = LdtkRootLoaderError;

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

            let assets_path: PathBuf = load_context.path().clone().into();
            let assets_path = assets_path
                .parent()
                .ok_or_else(|| {
                    error!("Unable to get parent directory of project file!");
                    LdtkRootLoaderError::BadAssetPath(load_context.path().clone().into())
                })?
                .to_str()
                .ok_or_else(|| {
                    error!("Path unable to be parsed into a utf-8 string!");
                    LdtkRootLoaderError::PathUnparsable(load_context.path().clone().into())
                })?;

            let value: ldtk_json::LdtkJson = {
                let mut bytes = Vec::new();
                reader.read_to_end(&mut bytes).await?;
                serde_json::from_slice(&bytes)?
            };

            let world_levels_associations: Vec<(World, &Vec<ldtk_json::Level>)> =
                if value.worlds.is_empty() {
                    vec![((&value).try_into()?, &value.levels)]
                } else {
                    value
                        .worlds
                        .iter()
                        .map(|value| Ok((value.try_into()?, &value.levels)))
                        .collect::<Result<_, WorldError>>()?
                };

            let mut worlds: HashMap<String, World> =
                HashMap::with_capacity(world_levels_associations.len());

            if !value.external_levels {
                for (mut world, values) in world_levels_associations {
                    world.levels = values
                        .iter()
                        .map(|value| Ok((value.iid.clone(), value.try_into()?)))
                        .collect::<Result<_, LevelError>>()?;
                    worlds.insert(world.iid.clone(), world);
                }
            } else {
                for (mut world, values) in world_levels_associations {
                    let mut levels: HashMap<String, Level> = HashMap::with_capacity(values.len());
                    for value in values {
                        let level_asset = load_context
                            .load_direct(AssetPath::parse(
                                ldtk_project_path_join(
                                    assets_path,
                                    value.external_rel_path.as_ref().ok_or_else(|| {
                                        error!("External level with ExternalRelPath as None!");
                                        LdtkRootLoaderError::BadExternalLevel
                                    })?,
                                )
                                .to_str()
                                .ok_or_else(|| {
                                    error!("Unable to parse joined string!");
                                    LdtkRootLoaderError::PathJoinUnparsable(
                                        value.external_rel_path.as_ref().unwrap().clone(),
                                    )
                                })?,
                            ))
                            .await?
                            .take::<LdtkLevel>()
                            .ok_or_else(|| {
                                error!("Failed to generate external level!");
                                LdtkRootLoaderError::BadExternalLevel
                            })?;
                        levels.insert(value.iid.clone(), Level::try_from(&level_asset.value)?);
                    }
                    world.levels = levels;
                    worlds.insert(world.iid.clone(), world);
                }
            }

            debug!(
                "LDtk root project file: {} loaded!",
                load_context.path().to_str().unwrap_or_default()
            );

            Ok(LdtkProject {
                value,
                worlds,
                assets_path: assets_path.to_string(),
            })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtk"]
    }
}
