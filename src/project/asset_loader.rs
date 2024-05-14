use bevy::asset::AssetLoader;
use bevy::asset::AsyncReadExt;
use bevy::asset::ReadAssetBytesError;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use std::path::Path;
use std::path::PathBuf;
use thiserror::Error;

use crate::ldtk;
use crate::level::LevelAsset;
use crate::project::ProjectStub;
use crate::util::ldtk_path_to_asset_path;
use crate::world::WorldAsset;

use crate::project::asset::ProjectAsset;
use crate::project::asset::ProjectResolver;

#[derive(Debug, Error)]
pub(crate) enum ProjectAssetLoaderError {
    #[error("IO error! {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON Parse error! {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("Read Asset Bytes error! {0}")]
    ReadAssetBytes(#[from] ReadAssetBytesError),
    #[error("None on a single world field! {0}")]
    NoneInSingleWorld(String),
    #[error("Could not get project directory? {0}")]
    BadProjectDirectory(PathBuf),
    #[error("external_rel_path is None when external levels enabled!")]
    NoneInExternalLevels,
}

#[derive(Debug, Default)]
pub(crate) struct ProjectAssetLoader;

impl AssetLoader for ProjectAssetLoader {
    type Asset = ProjectAsset;

    type Settings = ();

    type Error = ProjectAssetLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let asset_path = load_context.path().to_path_buf();

            info!("Loading LDtk project file: {asset_path:?}");

            let base_directory = asset_path
                .parent()
                .ok_or(ProjectAssetLoaderError::BadProjectDirectory(
                    asset_path.clone(),
                ))?
                .to_path_buf();

            let project_handle = load_context.load(load_context.path().to_path_buf());

            let value: ldtk::LdtkJson = {
                let mut bytes = Vec::new();
                reader.read_to_end(&mut bytes).await?;
                serde_json::from_slice(&bytes)?
            };

            let single_world = {
                if value.worlds.is_empty() {
                    vec![ldtk::World {
                        default_level_height: value.default_level_height.ok_or(
                            ProjectAssetLoaderError::NoneInSingleWorld(
                                "default_level_height".to_string(),
                            ),
                        )?,
                        default_level_width: value.default_level_width.ok_or(
                            ProjectAssetLoaderError::NoneInSingleWorld(
                                "default_level_width".to_string(),
                            ),
                        )?,
                        identifier: "World".to_string(), // TODO: Parameterize me, somehow!
                        iid: value.iid.clone(),
                        levels: value.levels.clone(),
                        world_grid_height: value.world_grid_height.ok_or(
                            ProjectAssetLoaderError::NoneInSingleWorld(
                                "world_grid_height".to_string(),
                            ),
                        )?,
                        world_grid_width: value.world_grid_width.ok_or(
                            ProjectAssetLoaderError::NoneInSingleWorld(
                                "world_grid_width".to_string(),
                            ),
                        )?,
                        world_layout: value.world_layout.clone(),
                    }]
                } else {
                    Vec::new()
                }
            };

            let mut external_levels = HashMap::default();

            if value.external_levels {
                for world in if value.worlds.is_empty() {
                    single_world.iter()
                } else {
                    value.worlds.iter()
                } {
                    for level_stub in &world.levels {
                        let level_path = level_stub
                            .external_rel_path
                            .as_ref()
                            .ok_or(ProjectAssetLoaderError::NoneInExternalLevels)?;
                        let level_path = Path::new(level_path);
                        let level_path = ldtk_path_to_asset_path(&base_directory, level_path);
                        let level_bytes = load_context.read_asset_bytes(level_path).await?;
                        let level_json = serde_json::from_slice(&level_bytes)?;

                        external_levels.insert(world.iid.clone(), level_json);
                    }
                }
            };

            let tileset_handles = value
                .defs
                .tilesets
                .iter()
                .filter_map(|tileset_definition| tileset_definition.rel_path.as_ref())
                .map(|ldtk_path| {
                    let asset_path = Path::new(&ldtk_path);
                    let asset_path = ldtk_path_to_asset_path(&base_directory, asset_path);
                    let asset_handle = load_context.load(asset_path);
                    (ldtk_path.clone(), asset_handle)
                })
                .collect();

            let project_stub = ProjectStub {
                value,
                single_world,
                external_levels,
            };

            let background_handles = project_stub
                .get_worlds()
                .flat_map(|world| world.levels.iter())
                .filter_map(|level| level.bg_rel_path.as_ref())
                .map(|ldtk_path| {
                    let asset_path = Path::new(&ldtk_path);
                    let asset_path = ldtk_path_to_asset_path(&base_directory, asset_path);
                    let asset_handle = load_context.load(asset_path);
                    (ldtk_path.clone(), asset_handle)
                })
                .collect();

            let mut world_handles = HashMap::default();
            let mut level_handles = HashMap::default();
            project_stub.get_worlds().for_each(|world| {
                let world_iid = world.iid.clone();

                let world_asset = WorldAsset {
                    project_handle: project_handle.clone(),
                    iid: world_iid.clone(),
                };

                let world_handle = load_context
                    .add_loaded_labeled_asset(world.identifier.clone(), world_asset.into());

                world_handles.insert(world_iid.clone(), world_handle);

                project_stub
                    .get_levels_by_world_iid(world_iid.clone().as_str())
                    .for_each(|level| {
                        let level_iid = level.iid.clone();

                        let level_asset = LevelAsset {
                            project_handle: project_handle.clone(),
                            iid: level_iid.clone(),
                        };

                        let tag = format!("{}/{}", world.identifier, level.identifier);

                        let level_handle =
                            load_context.add_loaded_labeled_asset(tag, level_asset.into());

                        level_handles.insert(level_iid, level_handle);
                    });
            });

            let project_asset = ProjectAsset {
                value: project_stub.value,
                single_world: project_stub.single_world,
                external_levels: project_stub.external_levels,
                world_handles,
                level_handles,
                tileset_handles,
                background_handles,
            };

            info!("LDtk project file loaded!: {asset_path:?}");

            Ok(project_asset)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtk"]
    }
}