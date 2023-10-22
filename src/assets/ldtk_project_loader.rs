use crate::assets::ldtk_level::LdtkLevel;
use crate::assets::ldtk_project::LdtkProject;
use crate::assets::structs::level::{Level, LevelError};
use crate::assets::structs::world::{World, WorldError};
use crate::assets::util::ldtk_file_to_asset_path;
use crate::ldtk_json;
use crate::util::ColorParseError;
use bevy::asset::{io::Reader, AssetLoader, LoadContext};
use bevy::asset::{AssetPath, AsyncReadExt};
use bevy::prelude::*;
use bevy::utils::thiserror;
use std::collections::HashMap;
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

            let value: ldtk_json::LdtkJson = {
                let mut bytes = Vec::new();
                reader.read_to_end(&mut bytes).await?;
                serde_json::from_slice(&bytes)?
            };

            let world_levels_associations: Vec<(World, &Vec<ldtk_json::Level>)> =
                if value.worlds.is_empty() {
                    vec![(World::try_from(&value)?, &value.levels)]
                } else {
                    value
                        .worlds
                        .iter()
                        .map(|value| Ok((World::try_from(value)?, &value.levels)))
                        .collect::<Result<_, WorldError>>()?
                };

            let mut worlds: HashMap<String, World> =
                HashMap::with_capacity(world_levels_associations.len());

            if !value.external_levels {
                for (mut world, values) in world_levels_associations {
                    world.levels = values
                        .iter()
                        .map(|value| Ok((value.iid.clone(), Level::try_from(value)?)))
                        .collect::<Result<_, LevelError>>()?;
                    worlds.insert(world.iid.clone(), world);
                }
            } else {
                for (mut world, values) in world_levels_associations {
                    let mut levels: HashMap<String, Level> = HashMap::with_capacity(values.len());
                    for value in values {
                        let level_asset = load_context
                            .load_direct(AssetPath::parse(
                                ldtk_file_to_asset_path(
                                    value.external_rel_path.as_ref().unwrap().as_str(),
                                    load_context.path(),
                                )
                                .as_str(),
                            ))
                            .await?
                            .take::<LdtkLevel>()
                            .ok_or(LdtkRootLoaderError::BadExternalLevel)?;
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
                _worlds: worlds,
            })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtk"]
    }
}
