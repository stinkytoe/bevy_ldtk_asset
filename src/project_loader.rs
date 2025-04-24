use std::path::Path;
use std::str::FromStr;

use bevy_asset::AssetLoader;
use bevy_asset::Handle;
use bevy_log::debug;
use bevy_platform::collections::HashMap;

use crate::Result;
use crate::asset_labels::ProjectAssetPath;
use crate::entity_definition::EntityDefinition;
use crate::enum_definition::EnumDefinition;
use crate::error::Error;
use crate::iid::Iid;
use crate::iid::IidMap;
use crate::iid::IidSet;
use crate::layer_definition::LayerDefinition;
use crate::ldtk;
use crate::ldtk_import_error;
use crate::project::Project;
use crate::tileset_definition::TilesetDefinition;
use crate::uid::UidMap;
use crate::world::World;

pub(crate) struct UniqueIidAuditor {
    known_iids: IidSet,
}

impl UniqueIidAuditor {
    pub(crate) fn new() -> Self {
        let known_iids = IidSet::default();

        Self { known_iids }
    }

    pub(crate) fn check(&mut self, iid: Iid) -> Result<()> {
        self.known_iids
            .insert(iid)
            .then_some(())
            .ok_or(Error::DuplicateIidError(iid))
    }
}

pub(crate) struct ProjectContext<'a> {
    pub(crate) project_directory: &'a Path,
    pub(crate) external_levels: bool,
}

pub(crate) struct ProjectDefinitionContext<'a> {
    pub(crate) tileset_definitions: &'a UidMap<Handle<TilesetDefinition>>,
    pub(crate) layer_definitions: &'a UidMap<Handle<LayerDefinition>>,
    pub(crate) entity_definitions: &'a UidMap<Handle<EntityDefinition>>,
    pub(crate) enum_definitions: &'a HashMap<String, Handle<EnumDefinition>>,
}

#[derive(Default)]
pub(crate) struct ProjectLoader;

impl AssetLoader for ProjectLoader {
    type Asset = Project;
    type Settings = ();
    type Error = crate::Error;

    async fn load(
        &self,
        reader: &mut dyn bevy_asset::io::Reader,
        _settings: &Self::Settings,
        load_context: &mut bevy_asset::LoadContext<'_>,
    ) -> Result<Self::Asset> {
        let ldtk_project: ldtk::LdtkProject = {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            serde_json::from_slice(&bytes)?
        };

        let project_path = load_context.path();

        let project_directory = project_path
            .parent()
            .ok_or(ldtk_import_error!("Unable to get project_directory!"))?
            .to_path_buf();

        let project_path = project_path
            .to_str()
            .ok_or(ldtk_import_error!(
                "Could not convert project path to str! given: {:?}",
                project_path
            ))?
            .to_string();

        let project_asset_path = ProjectAssetPath::new(&project_path)?;

        let mut unique_iid_auditor = UniqueIidAuditor::new();

        debug!("Loading LDtk project: {project_path}");

        let project_iid = Iid::from_str(&ldtk_project.iid)?;
        unique_iid_auditor.check(project_iid)?;

        let json_version = ldtk_project.json_version.clone();

        const SUPPORTED_VERSION: &str = "1.5.3";
        if json_version != SUPPORTED_VERSION {
            return Err(ldtk_import_error!(
                "Bad LDtk JSON version! expected: {SUPPORTED_VERSION} given: {json_version}"
            ));
        }

        // If the worlds vec is empty, then this is likely a single world LDtk project. To simplify
        // things, though, I only parse the world objects. So... in the event of a single world
        // project (the most common type), we simply create a vec of one element, with an
        // [ldtk::World] that we construct ourselves. We insert the name "World" as its identifier
        // as a default since projects, and therefore, single worlds do not have identifiers.
        let ldtk_worlds = if ldtk_project.worlds.is_empty() {
            &[ldtk::World {
                default_level_height: ldtk_project.default_level_height.ok_or(
                    ldtk_import_error!("default_level_height is None in single world project?"),
                )?,
                default_level_width: ldtk_project.default_level_width.ok_or(ldtk_import_error!(
                    "default_level_width is None in single world project?"
                ))?,
                identifier: "World".to_string(),
                iid: ldtk_project.iid,
                levels: ldtk_project.levels,
                world_grid_height: ldtk_project.world_grid_width.ok_or(ldtk_import_error!(
                    "world_grid_height is None in single world project?"
                ))?,
                world_grid_width: ldtk_project.world_grid_width.ok_or(ldtk_import_error!(
                    "world_grid_width is None in single world project?"
                ))?,
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
            .collect::<Result<_>>()?;

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
            .collect::<Result<_>>()?;

        let entity_definitions = ldtk_project
            .defs
            .entities
            .iter()
            .map(|ldtk_entity_definition| {
                EntityDefinition::create_handle_pair(
                    ldtk_entity_definition,
                    &project_asset_path,
                    load_context,
                    &tileset_definitions,
                )
            })
            .collect::<Result<_>>()?;

        let enum_definitions = ldtk_project
            .defs
            .enums
            .iter()
            .map(|ldtk_enum_definition| {
                EnumDefinition::create_handle_pair(
                    ldtk_enum_definition,
                    &project_asset_path,
                    load_context,
                    &project_context,
                    &tileset_definitions,
                )
            })
            .collect::<Result<_>>()?;

        let project_definitions_context = ProjectDefinitionContext {
            tileset_definitions: &tileset_definitions,
            layer_definitions: &layer_definitions,
            entity_definitions: &entity_definitions,
            enum_definitions: &enum_definitions,
        };

        let worlds = ldtk_worlds
            .iter()
            .map(|ldtk_world| {
                World::create_handle_pair(
                    ldtk_world,
                    &project_asset_path,
                    load_context,
                    &mut unique_iid_auditor,
                    &project_context,
                    &project_definitions_context,
                )
            })
            .collect::<Result<IidMap<Handle<World>>>>()?;

        debug!("Loading LDtk project completed! {project_path}");

        Ok(Project {
            iid: project_iid,
            json_version,
            path: project_path,
            worlds,
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtk"]
    }
}
