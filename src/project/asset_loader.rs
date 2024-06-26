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
use crate::layer::Tile;
use crate::ldtk;
use crate::level::LevelAsset;
use crate::level::NewLevelAssetError;
use crate::project::defs::EntityDefinitionFromError;
use crate::project::defs::EnumDefinition;
use crate::project::defs::LayerDefinitionFromError;
use crate::project::defs::TilesetDefinition;
use crate::project::ProjectAsset;
use crate::project::ProjectSettings;
use crate::util::bevy_color_from_ldtk;
use crate::util::ldtk_path_to_asset_path;
use crate::util::ColorParseError;
use crate::world::NewWorldAssetError;
use crate::world::WorldAsset;

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

                        let mut tiles = Vec::new();

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
                            SublayerToLoad::Tile(inner_tiles) => {
                                tiles = inner_tiles.iter().map(Tile::from).collect();
                            }
                        };

                        let asset = LayerAsset::new(
                            layer,
                            self_handle.clone(),
                            index,
                            tiles,
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
