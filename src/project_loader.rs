use std::path::Path;
use std::str::FromStr;

use bevy::asset::AssetLoader;
use bevy::asset::AsyncReadExt;
use bevy::log::debug;
use bevy::log::trace;
use bevy::reflect::List;
use bevy::reflect::Map;
use bevy::tasks::block_on;

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

            let project_path = load_context.path();

            let project_directory = project_path
                .parent()
                .ok_or(Error::LdtkImportError(
                    "Unable to get project_directory!".to_string(),
                ))?
                .to_path_buf();

            let project_path = project_path
                .to_str()
                .ok_or(Error::LdtkImportError(format!(
                    "Could not convert project path to str! given: {:?}",
                    project_path
                )))?
                .to_string();

            debug!("Loading LDtk project: {:?}", project_path);

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

            macro_rules! entity_inner {
                ($layer_iid:expr, $layer_path: expr, $layer_label: expr, $ldtk_entity:expr ) => {{
                    let entity = Entity::new($ldtk_entity, $layer_path)?;
                    let entity_label = format!(
                        "{}/{}@{}",
                        $layer_label, entity.identifier, $ldtk_entity.iid
                    );
                    let entity_iid = entity.iid;
                    parent_map.insert(entity_iid, $layer_iid);
                    trace!("Entity loaded: {entity_label}");

                    let entity_handle =
                        load_context.add_loaded_labeled_asset(entity_label, entity.into());
                    entities.insert(entity_iid, entity_handle);

                    Ok(())
                }};
            }

            macro_rules! layer_inner {
                ($level_iid:expr, $level_path:expr, $level_label:expr, $index:expr, $ldtk_layer:expr) => {{
                    let layer = Layer::new($ldtk_layer, $index, $level_path)?;
                    let layer_label = format!("{}/{}", $level_label, layer.identifier);
                    let layer_iid = layer.iid;
                    let layer_path = format!("{project_path}#{layer_label}");
                    parent_map.insert(layer_iid, $level_iid);
                    trace!("Layer loaded: {layer_label}");

                    $ldtk_layer.entity_instances.iter().try_for_each(
                        |ldtk_entity| -> Result<(), Error> {
                            entity_inner!(layer_iid, &layer_path, &layer_label, ldtk_entity)
                        },
                    )?;

                    let layer_handle =
                        load_context.add_loaded_labeled_asset(layer_label, layer.into());
                    layers.insert(layer_iid, layer_handle);

                    Ok(())
                }};
            }

            macro_rules! level_inner {
                ($world_iid:expr, $world_path:expr, $world_label:expr,  $ldtk_level:expr) => {{
                    let level = Level::new($ldtk_level, $world_path)?;
                    let level_label = format!("{}/{}", $world_label, level.identifier);
                    let level_iid = level.iid;
                    let level_path = format!("{project_path}#{level_label}");
                    parent_map.insert(level_iid, $world_iid);
                    trace!("Level loaded: {level_label}");

                    $ldtk_level
                        .layer_instances
                        .as_ref()
                        .ok_or(Error::LdtkImportError(
                            "layer_instances is None!".to_string(),
                        ))?
                        .iter()
                        .rev()
                        .enumerate()
                        .try_for_each(|(index, ldtk_layer)| -> Result<(), Error> {
                            layer_inner!(level_iid, &level_path, &level_label, index, ldtk_layer)
                        })?;

                    let level_handle =
                        load_context.add_loaded_labeled_asset(level_label, level.into());
                    levels.insert(level_iid, level_handle);

                    Ok(())
                }};
            }

            ldtk_worlds
                .iter()
                .try_for_each(|ldtk_world| -> Result<_, Error> {
                    let world = World::new(ldtk_world, &project_path)?;
                    let world_label = world.identifier.clone();
                    let world_iid = world.iid;
                    let world_path = format!("{project_path}#{world_label}");
                    parent_map.insert(world_iid, project_iid);
                    trace!("World loaded: {}", world_label);

                    let ldtk_levels = if ldtk_project.external_levels {
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

                    ldtk_levels
                        .iter()
                        .try_for_each(|ldtk_level| -> Result<(), Error> {
                            level_inner!(world_iid, &world_path, &world_label, ldtk_level)
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

            let mut children_map = IidMap::new();
            parent_map.iter().for_each(|(&child, &parent)| {
                children_map
                    .entry(parent)
                    .and_modify(|children: &mut Vec<Iid>| children.push(child))
                    .or_insert(vec![child]);
            });

            debug!("Loading LDtk project completed! {:?}", project_path);

            Ok(Project {
                iid: project_iid,
                json_version,
                worlds,
                levels,
                layers,
                entities,
                parent_map,
                children_map,
                tileset_images,
                tileset_definitions,
                enum_definitions,
            })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtk"]
    }
}
