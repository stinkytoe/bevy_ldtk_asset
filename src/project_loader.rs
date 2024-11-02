use std::path::Path;
use std::str::FromStr;

use bevy::asset::AssetLoader;
use bevy::asset::Handle;
use bevy::log::debug;

use crate::iid::Iid;
use crate::iid::IidMap;
use crate::label::ProjectAssetPath;
use crate::layer_definition::LayerDefinition;
use crate::ldtk;
use crate::project::Project;
use crate::tileset_definition::TilesetDefinition;
use crate::uid::UidMap;
use crate::world::World;

pub(crate) struct ProjectContext<'a> {
    pub(crate) project_directory: &'a Path,
    pub(crate) external_levels: bool,
}

pub(crate) struct ProjectDefinitionContext<'a> {
    pub(crate) tileset_definitions: &'a UidMap<Handle<TilesetDefinition>>,
    pub(crate) layer_definitions: &'a UidMap<Handle<LayerDefinition>>,
}

#[derive(Default)]
pub(crate) struct ProjectLoader;

impl AssetLoader for ProjectLoader {
    type Asset = Project;
    type Settings = ();
    type Error = crate::Error;

    async fn load(
        &self,
        reader: &mut dyn bevy::asset::io::Reader,
        _settings: &(),
        load_context: &mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let ldtk_project: ldtk::LdtkProject = {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            serde_json::from_slice(&bytes)?
        };

        let project_path = load_context.path();

        let project_directory = project_path
            .parent()
            .ok_or(crate::Error::LdtkImportError(
                "Unable to get project_directory!".to_string(),
            ))?
            .to_path_buf();

        let project_path = project_path
            .to_str()
            .ok_or(crate::Error::LdtkImportError(format!(
                "Could not convert project path to str! given: {:?}",
                project_path
            )))?
            .to_string();

        let project_asset_path = ProjectAssetPath::new(&project_path);

        debug!("Loading LDtk project: {project_path}");

        let project_iid = Iid::from_str(&ldtk_project.iid)?;

        let json_version = ldtk_project.json_version.clone();

        const SUPPORTED_VERSION: &str = "1.5.3";
        if json_version != SUPPORTED_VERSION {
            return Err(crate::Error::LdtkImportError(format!(
                "Bad LDtk JSON version! expected: {SUPPORTED_VERSION} given: {json_version}"
            )));
        }

        // If the worlds vec is empty, then this is likely a single world LDtk project. To simplify
        // things, though, I only parse the world objects. So... in the event of a single world
        // project (the most common type), we simply create a vec of one element, with an
        // [ldtk::World] that we construct ourselves. We insert the name "World" as its identifier
        // as a default since projects, and therefore, single worlds do not have identifiers.
        let ldtk_worlds = if ldtk_project.worlds.is_empty() {
            &[ldtk::World {
                default_level_height: ldtk_project.default_level_height.ok_or(
                    crate::Error::LdtkImportError(
                        "default_level_height is None in single world project?".to_string(),
                    ),
                )?,
                default_level_width: ldtk_project.default_level_width.ok_or(
                    crate::Error::LdtkImportError(
                        "default_level_width is None in single world project?".to_string(),
                    ),
                )?,
                identifier: "World".to_string(),
                iid: ldtk_project.iid,
                levels: ldtk_project.levels,
                world_grid_height: ldtk_project.world_grid_width.ok_or(
                    crate::Error::LdtkImportError(
                        "world_grid_height is None in single world project?".to_string(),
                    ),
                )?,
                world_grid_width: ldtk_project.world_grid_width.ok_or(
                    crate::Error::LdtkImportError(
                        "world_grid_width is None in single world project?".to_string(),
                    ),
                )?,
                world_layout: ldtk_project.world_layout,
            }]
        } else {
            ldtk_project.worlds.as_slice()
        };

        let project_context = ProjectContext {
            project_directory: &project_directory,
            external_levels: ldtk_project.external_levels,
        };

        let tileset_definitions = ldtk_project
            .defs
            .tilesets
            .iter()
            .map(|ldtk_tileset_definition| {
                TilesetDefinition::create_handle_pair(
                    ldtk_tileset_definition,
                    &project_asset_path,
                    load_context,
                    &project_context,
                )
            })
            .collect::<crate::Result<_>>()?;

        let layer_definitions = ldtk_project
            .defs
            .layers
            .iter()
            .map(|ldtk_layer_definition| {
                LayerDefinition::create_handle_pair(
                    ldtk_layer_definition,
                    &project_asset_path,
                    load_context,
                    &tileset_definitions,
                )
            })
            .collect::<crate::Result<_>>()?;

        let project_definitions_context = ProjectDefinitionContext {
            tileset_definitions: &tileset_definitions,
            layer_definitions: &layer_definitions,
        };

        let worlds = ldtk_worlds
            .iter()
            .map(|ldtk_world| {
                World::create_handle_pair(
                    ldtk_world,
                    &project_asset_path,
                    load_context,
                    &project_context,
                    &project_definitions_context,
                )
            })
            .collect::<crate::Result<IidMap<Handle<World>>>>()?;

        debug!("Loading LDtk project completed! {project_path}");

        Ok(Project {
            iid: project_iid,
            json_version,
            worlds,
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtk"]
    }
}
