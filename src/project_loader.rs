use std::path::Path;
use std::str::FromStr;

use bevy::asset::AssetLoader;
use bevy::asset::AsyncReadExt;
use bevy::asset::Handle;
use bevy::log::debug;

use crate::iid::Iid;
use crate::iid::IidMap;
use crate::label::ProjectAssetPath;
use crate::ldtk;
use crate::project::Project;
use crate::world::World;

pub(crate) struct ProjectContext<'a> {
    pub(crate) project_directory: &'a Path,
    pub(crate) external_levels: bool,
}

#[derive(Default)]
pub(crate) struct ProjectLoader;

impl AssetLoader for ProjectLoader {
    type Asset = Project;

    type Settings = ();

    type Error = crate::Error;

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

            let worlds = ldtk_worlds
                .iter()
                .map(|ldtk_world| {
                    World::create_handle_pair(
                        ldtk_world,
                        &project_asset_path,
                        load_context,
                        &project_context,
                    )
                })
                .collect::<crate::Result<IidMap<Handle<World>>>>()?;

            debug!("Loading LDtk project completed! {project_path}");

            Ok(Project {
                iid: project_iid,
                json_version,
                worlds,
            })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtk"]
    }
}
