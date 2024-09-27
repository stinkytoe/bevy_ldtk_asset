use std::path::Path;
use std::str::FromStr;

use bevy::asset::AssetLoader;
use bevy::asset::AsyncReadExt;
use bevy::log::debug;
use bevy::log::trace;
use bevy::reflect::List;
use bevy::reflect::Map;
use bevy::tasks::block_on;
use bevy::utils::HashMap;

use crate::entity::Entity;
use crate::error::Error;
use crate::iid::Iid;
use crate::iid::IidMap;
use crate::layer::Layer;
use crate::ldtk;
use crate::ldtk_path::ldtk_path_to_bevy_path;
use crate::level::Level;
use crate::project::Project;
use crate::world::World;

#[derive(Default)]
pub(crate) struct ProjectLoader;

impl AssetLoader for ProjectLoader {
    type Asset = Project;

    type Settings = ();

    type Error = Error;

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> impl bevy::utils::ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let ldtk_project: ldtk::LdtkProject = {
                let mut bytes = Vec::new();
                reader.read_to_end(&mut bytes).await?;
                serde_json::from_slice(&bytes)?
            };

            let project_directory = load_context
                .path()
                .parent()
                .ok_or(Error::LdtkImportError(
                    "Unable to get project_directory!".to_string(),
                ))?
                .to_path_buf();

            debug!("Loading LDtk project: {:?}", load_context.path());

            let project_iid = Iid::from_str(&ldtk_project.iid)?;

            let json_version = ldtk_project.json_version.clone();

            if json_version != "1.5.3" {
                return Err(Error::LdtkImportError(format!(
                    "Bad LDtk JSON version! given: {json_version}"
                )));
            }

            let ldtk_worlds = if ldtk_project.worlds.is_empty() {
                vec![ldtk::World {
                    default_level_height: ldtk_project.default_level_height.ok_or(
                        Error::LdtkImportError(
                            "default_level_height is None in single world project?".to_string(),
                        ),
                    )?,
                    default_level_width: ldtk_project.default_level_width.ok_or(
                        Error::LdtkImportError(
                            "default_level_width is None in single world project?".to_string(),
                        ),
                    )?,
                    identifier: "World".to_string(),
                    iid: ldtk_project.iid,
                    levels: ldtk_project.levels,
                    world_grid_height: ldtk_project.world_grid_width.ok_or(
                        Error::LdtkImportError(
                            "world_grid_height is None in single world project?".to_string(),
                        ),
                    )?,
                    world_grid_width: ldtk_project.world_grid_width.ok_or(
                        Error::LdtkImportError(
                            "world_grid_width is None in single world project?".to_string(),
                        ),
                    )?,
                    world_layout: ldtk_project.world_layout,
                }]
            } else {
                ldtk_project.worlds
            };

            let mut worlds = IidMap::new();
            let mut levels = IidMap::new();
            let mut layers = IidMap::new();
            let mut entities = IidMap::new();
            let mut parent_map = IidMap::new();

            ldtk_worlds
                .iter()
                .try_for_each(|ldtk_world| -> Result<_, Error> {
                    let world = World::new(ldtk_world)?;
                    let world_label = world.identifier.clone();
                    let world_iid = world.iid;
                    parent_map.insert(world_iid, project_iid);
                    trace!("World loaded: {}", world_label);

                    let level_vec = if ldtk_project.external_levels {
                        &ldtk_world
                            .levels
                            .iter()
                            .map(|ldtk_level| -> Result<ldtk::Level, Error> {
                                let external_rel_path = ldtk_level
                                    .external_rel_path
                                    .as_ref()
                                    .ok_or(Error::LdtkImportError(
                                        "external_rel_path is None when external_levels is true!"
                                            .to_string(),
                                    ))?;

                                trace!("Attempting to load external level from path: {external_rel_path}");

                                let ldtk_path = Path::new(external_rel_path);
                                let bevy_path =
                                    ldtk_path_to_bevy_path(&project_directory, ldtk_path);
                                let bytes = block_on(async {
                                    load_context.read_asset_bytes(bevy_path).await
                                })?;
                                let level: ldtk::Level = serde_json::from_slice(&bytes)?;
                                Ok(level)
                            })
                            .collect::<Result<_, _>>()?
                    } else {
                        &ldtk_world.levels
                    };

                    level_vec
                        .iter()
                        .try_for_each(|ldtk_level| -> Result<(), Error> {
                            let level = Level::new(ldtk_level)?;
                            let level_label = format!("{world_label}/{}", level.identifier);
                            let level_iid = level.iid;
                            parent_map.insert(level_iid, world_iid);
                            trace!("Level loaded: {level_label}");

                            ldtk_level
                                .layer_instances
                                .as_ref()
                                .ok_or(Error::LdtkImportError(
                                    "layer_instances is None!".to_string(),
                                ))?
                                .iter()
                                .try_for_each(|ldtk_layer| -> Result<(), Error> {
                                    let layer = Layer::new(ldtk_layer)?;
                                    let layer_label = format!("{level_label}/{}", layer.identifier);
                                    let layer_iid = layer.iid;
                                    parent_map.insert(layer_iid, level_iid);
                                    trace!("Layer loaded: {layer_label}");

                                    ldtk_layer.entity_instances.iter().try_for_each(
                                        |ldtk_entity| -> Result<(), Error> {
                                            let entity = Entity::new(ldtk_entity)?;
                                            let entity_label = format!(
                                                "{layer_label}/{}@{}",
                                                entity.identifier, ldtk_entity.iid
                                            );
                                            let entity_iid = entity.iid;
                                            parent_map.insert(entity_iid, layer_iid);
                                            trace!("Entity loaded: {entity_label}");

                                            let entity_handle = load_context
                                                .add_loaded_labeled_asset(
                                                    entity_label,
                                                    entity.into(),
                                                );
                                            entities.insert(entity_iid, entity_handle);

                                            Ok(())
                                        },
                                    )?;

                                    let layer_handle = load_context
                                        .add_loaded_labeled_asset(layer_label, layer.into());
                                    layers.insert(layer_iid, layer_handle);

                                    Ok(())
                                })?;

                            let level_handle =
                                load_context.add_loaded_labeled_asset(level_label, level.into());
                            levels.insert(level_iid, level_handle);

                            Ok(())
                        })?;

                    let world_handle =
                        load_context.add_loaded_labeled_asset(world_label, world.into());
                    worlds.insert(world_iid, world_handle);

                    Ok(())
                })?;

            let tileset_images = ldtk_project
                .defs
                .tilesets
                .iter()
                .filter_map(|tileset| tileset.rel_path.as_deref())
                .map(|ldtk_path_str| (ldtk_path_str, Path::new(ldtk_path_str)))
                .map(|(ldtk_path_str, ldtk_path)| {
                    (
                        ldtk_path_str.to_string(),
                        ldtk_path_to_bevy_path(&project_directory, ldtk_path),
                    )
                })
                .map(|(ldtk_path_str, bevy_path)| (ldtk_path_str, load_context.load(bevy_path)))
                .collect();

            let tileset_definitions = ldtk_project
                .defs
                .tilesets
                .into_iter()
                .map(|tileset_definition| tileset_definition.into())
                .collect();

            let enum_definitions = ldtk_project
                .defs
                .enums
                .into_iter()
                .map(|enum_definition| enum_definition.into())
                .collect();

            let mut children_map: IidMap<Vec<Iid>> = IidMap::default();
            parent_map.iter().for_each(|(child, parent)| {
                match children_map.get_mut(parent) {
                    Some(children) => {
                        children.push(*child);
                    }
                    None => {
                        children_map.insert(*parent, vec![*child]);
                    }
                };
            });

            debug!("Loading LDtk project completed! {:?}", load_context.path());

            Ok(Project {
                iid: project_iid,
                json_version,
                worlds,
                levels,
                layers,
                entities,
                tileset_images,
                tileset_definitions,
                enum_definitions,
                parent_map,
                children_map,
            })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtk"]
    }
}
