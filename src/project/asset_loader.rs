use bevy::asset::AssetLoader;
use bevy::asset::AsyncReadExt;
use bevy::asset::ReadAssetBytesError;
use bevy::prelude::*;
use bevy::utils::HashMap;
use std::path::Path;
use std::path::PathBuf;
use thiserror::Error;

use crate::entity::EntityAsset;
use crate::entity::NewEntityAssetError;
use crate::layer::LayerAsset;
use crate::layer::LayerType;
use crate::layer::LayerTypeError;
use crate::ldtk;
use crate::level::LevelAsset;
use crate::level::NewLevelAssetError;
use crate::prelude::defs::EnumDefinition;
use crate::prelude::defs::LayerDefinition;
use crate::prelude::defs::TilesetDefinition;
use crate::project::ProjectAsset;
use crate::project::ProjectSettings;
use crate::util::bevy_color_from_ldtk;
use crate::util::ldtk_path_to_asset_path;
use crate::util::ColorParseError;
use crate::world::NewWorldAssetError;
use crate::world::WorldAsset;

use super::defs::EntityDefinitionFromError;
use super::defs::LayerDefinitionFromError;

#[derive(Debug, Error)]
pub(crate) enum ProjectAssetLoaderError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    ColorParseError(#[from] ColorParseError),
    #[error(transparent)]
    NewWorldAssetError(#[from] NewWorldAssetError),
    #[error(transparent)]
    NewLevelAssetError(#[from] NewLevelAssetError),
    #[error(transparent)]
    ReadAssetBytesError(#[from] ReadAssetBytesError),
    #[error(transparent)]
    LayerTypeError(#[from] LayerTypeError),
    #[error(transparent)]
    NewEntityAssetError(#[from] NewEntityAssetError),
    #[error(transparent)]
    LayerDefinitionFromError(#[from] LayerDefinitionFromError),
    #[error(transparent)]
    EntityDefinitionFromError(#[from] EntityDefinitionFromError),
    #[error("Could not get project directory? {0}")]
    BadProjectDirectory(PathBuf),
    #[error("externalRelPath is None when external levels is true?")]
    ExternalRelPathIsNone,
    #[error("entity instances in non-entity type layer!")]
    NonEntityLayerWithEntities,
    #[error("tile instances in entity type layer!")]
    NonTileLayerWithTiles,
    #[error("Value is None in a single world project?")]
    ValueMissingInSingleWorld,
    #[error("Layer Instances is None in a non-external levels project?")]
    LayerInstancesIsNone,
    #[error("Int Grid/Auto Layer should only have auto tiles!")]
    IntGridWithEntitiesOrGridTiles,
    #[error("Tiles Layer should only have grid tiles!")]
    TilesWithAutoLayerOrEntities,
}

#[derive(Debug, Default)]
pub(crate) struct ProjectAssetLoader;

impl AssetLoader for ProjectAssetLoader {
    type Asset = ProjectAsset;

    type Settings = ProjectSettings;

    type Error = ProjectAssetLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        settings: &'a Self::Settings,
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

            let self_handle: Handle<ProjectAsset> =
                load_context.load(load_context.path().to_path_buf());

            let value: ldtk::LdtkJson = {
                let mut bytes = Vec::new();
                reader.read_to_end(&mut bytes).await?;
                serde_json::from_slice(&bytes)?
            };

            let mut world_assets_by_identifier = HashMap::default();
            let mut world_assets_by_iid = HashMap::default();

            let worlds = if value.worlds.is_empty() {
                vec![ldtk::World {
                    default_level_height: value
                        .default_level_height
                        .ok_or(ProjectAssetLoaderError::ValueMissingInSingleWorld)?,
                    default_level_width: value
                        .default_level_width
                        .ok_or(ProjectAssetLoaderError::ValueMissingInSingleWorld)?,
                    identifier: "World".into(),
                    iid: value.iid.clone(),
                    levels: value.levels,
                    world_grid_height: value
                        .world_grid_height
                        .ok_or(ProjectAssetLoaderError::ValueMissingInSingleWorld)?,
                    world_grid_width: value
                        .world_grid_width
                        .ok_or(ProjectAssetLoaderError::ValueMissingInSingleWorld)?,
                    world_layout: value.world_layout,
                }]
            } else {
                value.worlds
            };

            let tile_definitions: HashMap<i64, String> = value
                .defs
                .tilesets
                .iter()
                .filter_map(|tileset_def| {
                    tileset_def
                        .rel_path
                        .as_ref()
                        .map(|path| (tileset_def.uid, path.clone()))
                })
                .collect();

            for world in worlds.iter() {
                let mut level_assets_by_identifier = HashMap::default();
                let mut level_assets_by_iid = HashMap::default();

                for level in world.levels.iter() {
                    let mut layer_assets_by_identifier = HashMap::default();
                    let mut layer_assets_by_iid = HashMap::default();

                    let layers = if value.external_levels {
                        let level_path = level
                            .external_rel_path
                            .as_ref()
                            .ok_or(ProjectAssetLoaderError::ExternalRelPathIsNone)?;
                        let level_path = Path::new(&level_path);
                        let level_path = ldtk_path_to_asset_path(&base_directory, level_path);
                        let bytes = load_context.read_asset_bytes(level_path).await?;
                        let level_json: ldtk::Level = serde_json::from_slice(&bytes)?;
                        level_json.layer_instances.unwrap()
                    } else {
                        level
                            .layer_instances
                            .as_ref()
                            .ok_or(ProjectAssetLoaderError::LayerInstancesIsNone)?
                            .to_vec()
                    };

                    for (index, layer) in layers.iter().rev().enumerate() {
                        let mut entity_assets_by_identifier = HashMap::default();
                        let mut entity_assets_by_iid = HashMap::default();

                        let layer_type = LayerType::new(&layer.layer_instance_type)?;

                        enum SublayerToLoad<'a> {
                            Entity(&'a Vec<ldtk::EntityInstance>),
                            Tile(&'a Vec<ldtk::TileInstance>),
                        }

                        let sublayer_to_spawn = match (
                            layer_type,
                            layer.grid_tiles.len(),
                            layer.auto_layer_tiles.len(),
                            layer.entity_instances.len(),
                        ) {
                            (LayerType::Entities, g, a, _) if g != 0 || a != 0 => {
                                return Err(ProjectAssetLoaderError::NonTileLayerWithTiles);
                            }
                            (LayerType::Entities, _, _, _) => {
                                SublayerToLoad::Entity(&layer.entity_instances)
                            }

                            (LayerType::IntGrid, g, _, e) | (LayerType::Autolayer, g, _, e)
                                if g != 0 || e != 0 =>
                            {
                                return Err(
                                    ProjectAssetLoaderError::IntGridWithEntitiesOrGridTiles,
                                );
                            }
                            (LayerType::IntGrid, _, _, _) | (LayerType::Autolayer, _, _, _) => {
                                SublayerToLoad::Tile(&layer.auto_layer_tiles)
                            }

                            (LayerType::Tiles, _, a, e) if a != 0 || e != 0 => {
                                return Err(ProjectAssetLoaderError::TilesWithAutoLayerOrEntities);
                            }
                            (LayerType::Tiles, _, _, _) => SublayerToLoad::Tile(&layer.grid_tiles),
                        };

                        match sublayer_to_spawn {
                            SublayerToLoad::Entity(entities) => {
                                for entity in entities.iter() {
                                    let asset = EntityAsset::new(entity, self_handle.clone())?;

                                    let handle = load_context.add_loaded_labeled_asset(
                                        format!(
                                            "{}/{}/{}/{}",
                                            world.identifier,
                                            level.identifier,
                                            layer.identifier,
                                            entity.identifier,
                                        ),
                                        asset.into(),
                                    );

                                    entity_assets_by_identifier
                                        .insert(entity.identifier.clone(), handle.clone());
                                    entity_assets_by_iid.insert(entity.iid.clone(), handle.clone());

                                    debug!("Added new EntityAsset!");
                                    debug!("identifier: {}", entity.identifier);
                                    debug!("iid: {}", entity.iid);
                                }
                            }
                            SublayerToLoad::Tile(_) => {}
                        };

                        let asset = LayerAsset::new(
                            layer,
                            self_handle.clone(),
                            index,
                            entity_assets_by_identifier,
                            entity_assets_by_iid,
                        )?;

                        let handle = load_context.add_loaded_labeled_asset(
                            format!(
                                "{}/{}/{}",
                                world.identifier, level.identifier, layer.identifier
                            ),
                            asset.into(),
                        );

                        layer_assets_by_identifier.insert(layer.identifier.clone(), handle.clone());
                        layer_assets_by_iid.insert(layer.iid.clone(), handle.clone());

                        debug!("Added new LayerAsset!");
                        debug!("identifier: {}", layer.identifier);
                        debug!("iid: {}", layer.iid);
                    }

                    let asset = LevelAsset::new(
                        level,
                        self_handle.clone(),
                        layer_assets_by_identifier,
                        layer_assets_by_iid,
                    )?;

                    let handle = load_context.add_loaded_labeled_asset(
                        format!("{}/{}", world.identifier, level.identifier),
                        asset.into(),
                    );

                    level_assets_by_identifier.insert(level.identifier.clone(), handle.clone());
                    level_assets_by_iid.insert(level.iid.clone(), handle);

                    debug!("Added new LevelAsset!");
                    debug!("identifier: {}", level.identifier);
                    debug!("iid: {}", level.iid);
                }

                let asset = WorldAsset::new(
                    world,
                    self_handle.clone(),
                    level_assets_by_identifier,
                    level_assets_by_iid,
                )?;

                let handle =
                    load_context.add_loaded_labeled_asset(world.identifier.clone(), asset.into());

                world_assets_by_identifier.insert(world.identifier.clone(), handle.clone());
                world_assets_by_iid.insert(world.iid.clone(), handle);

                debug!("Added new WorldAsset!");
                debug!("identifier: {}", world.identifier);
                debug!("iid: {}", world.iid);
            }

            let background_assets = worlds
                .iter()
                .flat_map(|world| world.levels.iter())
                .filter_map(|level| level.bg_rel_path.as_ref())
                .map(|ldtk_path| {
                    let asset_path = Path::new(&ldtk_path);
                    let asset_path = ldtk_path_to_asset_path(&base_directory, asset_path);
                    let asset_handle = load_context.load(asset_path);
                    (ldtk_path.clone(), asset_handle)
                })
                .collect();

            // let world_tuples: Vec<_> = if value.worlds.is_empty() {
            //     vec![(
            //         WorldAsset::new_from_project(&value, self_handle.clone())?,
            //         "World".to_owned(),
            //         &value.levels,
            //     )]
            // } else {
            //     value
            //         .worlds
            //         .iter()
            //         .map(|world| {
            //             Ok((
            //                 WorldAsset::new_from_world(world, self_handle.clone())?,
            //                 world.identifier.clone(),
            //                 &world.levels,
            //             ))
            //         })
            //         .collect::<Result<Vec<_>, ProjectAssetLoaderError>>()?
            // };
            //
            // let background_assets = world_tuples
            //     .iter()
            //     .flat_map(|(_, _, levels)| levels.iter())
            //     .filter_map(|level| level.bg_rel_path.as_ref())
            //     .map(|ldtk_path| {
            //         let asset_path = Path::new(&ldtk_path);
            //         let asset_path = ldtk_path_to_asset_path(&base_directory, asset_path);
            //         let asset_handle = load_context.load(asset_path);
            //         (ldtk_path.clone(), asset_handle)
            //     })
            //     .collect();
            //
            // for (world_asset, world_identifier, levels) in world_tuples {
            //     let iid = world_asset.iid.clone();
            //     let world_handle = load_context
            //         .add_loaded_labeled_asset(world_identifier.clone(), world_asset.into());
            //
            //     debug!("Added new WorldAsset!");
            //     debug!("identifier: {world_identifier}");
            //     debug!("iid: {iid}");
            //
            //     world_assets_by_identifier.insert(world_identifier.clone(), world_handle.clone());
            //     world_assets_by_iid.insert(iid, world_handle);
            //
            //     for level in levels.iter() {
            //         let iid = level.iid.clone();
            //         let level_asset = LevelAsset::new(level, self_handle.clone())?;
            //         let level_label = format!("{}/{}", world_identifier, level.identifier);
            //         let level_handle =
            //             load_context.add_loaded_labeled_asset(level_label, level_asset.into());
            //
            //         debug!("Added new LevelAsset!");
            //         debug!("identifier: {}", level.identifier);
            //         debug!("iid: {iid}");
            //
            //         level_assets_by_identifier
            //             .insert(level.identifier.clone(), level_handle.clone());
            //         level_assets_by_iid.insert(iid, level_handle);
            //
            //         let Some(layer_instances) = ({
            //             if !value.external_levels {
            //                 level.layer_instances.clone()
            //             } else {
            //                 let level_path = level
            //                     .external_rel_path
            //                     .as_ref()
            //                     .ok_or(ProjectAssetLoaderError::ExternalRelPathIsNone)?;
            //                 let level_path = Path::new(level_path);
            //                 let level_path = ldtk_path_to_asset_path(&base_directory, level_path);
            //                 let bytes = load_context.read_asset_bytes(level_path).await?;
            //                 let level_json: ldtk::Level = serde_json::from_slice(&bytes)?;
            //                 level_json.layer_instances
            //             }
            //         }) else {
            //             break;
            //         };
            //
            //         for layer_instance in layer_instances.iter().rev() {
            //             let iid = layer_instance.iid.clone();
            //             let layer_asset = LayerAsset::new(layer_instance, self_handle.clone())?;
            //             let layer_type = layer_asset.layer_type;
            //             let layer_label = format!(
            //                 "{}/{}/{}",
            //                 world_identifier, level.identifier, layer_instance.identifier
            //             );
            //             let layer_handle =
            //                 load_context.add_loaded_labeled_asset(layer_label, layer_asset.into());
            //
            //             debug!("Added new LayerAsset!");
            //             debug!("identifier: {}", layer_instance.identifier);
            //             debug!("iid: {iid}");
            //
            //             layer_assets_by_identifier
            //                 .insert(layer_instance.identifier.clone(), layer_handle.clone());
            //             layer_assets_by_iid.insert(iid, layer_handle);
            //
            //             match (
            //                 layer_type,
            //                 layer_instance.entity_instances.len(),
            //                 layer_instance.grid_tiles.len(),
            //                 layer_instance.auto_layer_tiles.len(),
            //             ) {
            //                 (LayerType::Tiles, n, _, _)
            //                 | (LayerType::IntGrid, n, _, _)
            //                 | (LayerType::Autolayer, n, _, _)
            //                     if n != 0 =>
            //                 {
            //                     return Err(ProjectAssetLoaderError::NonEntityLayerWithEntities);
            //                 }
            //                 (LayerType::Entities, _, n, m) if n != 0 || m != 0 => {
            //                     return Err(ProjectAssetLoaderError::NonTileLayerWithTiles);
            //                 }
            //                 (LayerType::Entities, _, _, _) => {
            //                     for entity_instance in &layer_instance.entity_instances {
            //                         let iid = entity_instance.iid.clone();
            //                         let entity_asset =
            //                             EntityAsset::new(entity_instance, self_handle.clone())?;
            //                         let entity_label = format!(
            //                             "{}/{}/{}/{}",
            //                             world_identifier,
            //                             level.identifier,
            //                             layer_instance.identifier,
            //                             entity_instance.identifier
            //                         );
            //                         let entity_handle = load_context.add_loaded_labeled_asset(
            //                             entity_label,
            //                             entity_asset.into(),
            //                         );
            //
            //                         debug!("Added new EntityAsset!");
            //                         debug!("identifier: {}", entity_instance.identifier);
            //                         debug!("iid: {iid}");
            //
            //                         entity_assets_by_identifier.insert(
            //                             entity_instance.identifier.clone(),
            //                             entity_handle.clone(),
            //                         );
            //                         entity_assets_by_iid.insert(iid, entity_handle);
            //                     }
            //                 }
            //                 _ => (),
            //             }
            //         }
            //     }
            // }
            //
            let tileset_assets = value
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

            let layer_defs = value
                .defs
                .layers
                .iter()
                .map(|layer_def| -> Result<_, LayerDefinitionFromError> {
                    Ok((layer_def.uid, layer_def.try_into()?))
                })
                .collect::<Result<_, _>>()?;

            let entity_defs = value
                .defs
                .entities
                .iter()
                .map(|entity_def| -> Result<_, EntityDefinitionFromError> {
                    Ok((entity_def.uid, entity_def.try_into()?))
                })
                .collect::<Result<_, _>>()?;

            let tileset_defs = value
                .defs
                .tilesets
                .iter()
                .map(TilesetDefinition::from)
                .map(|tileset_def| (tileset_def.uid, tileset_def))
                .collect();

            let enum_defs = value
                .defs
                .enums
                .iter()
                .map(EnumDefinition::from)
                .map(|enum_def| (enum_def.uid, enum_def))
                .collect();

            Ok(ProjectAsset {
                bg_color: bevy_color_from_ldtk(&value.bg_color)?,
                external_levels: value.external_levels,
                iid: value.iid.clone(),
                json_version: value.json_version.clone(),
                world_assets_by_identifier,
                world_assets_by_iid,
                tileset_assets,
                background_assets,
                layer_defs,
                entity_defs,
                tileset_defs,
                enum_defs,
                settings: settings.clone(),
                self_handle,
            })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtk"]
    }
}
