//! The LDtk project top level representation!

mod construct_entity_definitions;
mod construct_enum_definitions;
mod construct_layer_definitions;
mod construct_tileset_definitions;
mod construct_worlds_from_world_json;

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;

use bevy_asset::{Asset, Handle, LoadContext};
use bevy_platform::collections::HashMap;
use bevy_reflect::Reflect;

use crate::entity_definition::EntityDefinition;
use crate::enum_definition::EnumDefinition;
use crate::iid::Iid;
use crate::iid::IidMap;
use crate::layer_definition::LayerDefinition;
use crate::ldtk;
use crate::ldtk_asset_trait::LdtkAsset;
use crate::ldtk_import_error;
use crate::result::LdtkResult;
use crate::tileset_definition::TilesetDefinition;
use crate::uid::UidMap;
use crate::world::World;

use construct_entity_definitions::construct_entity_definitions;
use construct_enum_definitions::construct_enum_definitions;
use construct_layer_definitions::construct_layer_definitions;
use construct_tileset_definitions::construct_tileset_definitions;
use construct_worlds_from_world_json::construct_worlds_from_world_json;

/// This asset represents the entirety of an LDtk project file.
///
/// All referenced files (tilemaps, etc) will have assets created with their asset
/// labels referencing this top level asset.
///
/// External levels, if present, will be parsed along with this top level definition and included
/// as sub assets of this.
///
/// See [LDtk Project](https://ldtk.io/json/#ldtk-ProjectJson) for a full description.
#[derive(Debug, Asset, Reflect)]
pub struct Project {
    /// A unique [Iid] representing this entire project.
    pub iid: Iid,
    /// The version of the LDtk tool used to create this project.
    pub ldtk_version: String,

    // The global collection of tileset definitions.
    // pub tileset_definitions: UidMap<Handle<TilesetDefinition>>,
    /// The associated worlds in this project, indexed by their [Iid]s.
    ///
    /// This is the top level of the entire sub asset heirarchy.
    pub worlds: IidMap<Handle<World>>,
}

#[allow(unused)] // DELETE ME!
pub(crate) struct ProjectContext {
    pub(crate) tileset_definitions: UidMap<Handle<TilesetDefinition>>,
    pub(crate) layer_definitions: UidMap<Handle<LayerDefinition>>,
    pub(crate) enum_definitions: HashMap<String, Handle<EnumDefinition>>,
    pub(crate) entity_definitions: UidMap<Handle<EntityDefinition>>,
    pub(crate) external_levels: bool,
    pub(crate) project_directory: PathBuf,
}

impl Project {
    const SUPPORTED_VERSION: &'static str = "1.5.3";

    pub(crate) async fn new(
        project_json: ldtk::LdtkProject,
        load_context: &mut LoadContext<'_>,
    ) -> LdtkResult<Self> {
        let iid: Iid = project_json.iid.clone().try_into()?;

        let ldtk_version = project_json.json_version.clone();
        if ldtk_version != Self::SUPPORTED_VERSION {
            return Err(ldtk_import_error!(
                "Bad LDtk JSON version! expected: {} given: {}",
                Self::SUPPORTED_VERSION,
                ldtk_version
            ));
        }

        let project_directory = load_context
            .path()
            .parent()
            .ok_or(ldtk_import_error!("Unable to get project_directory!"))?
            .to_path_buf();

        let tileset_definitions = construct_tileset_definitions(
            project_json.defs.tilesets,
            &project_directory,
            load_context,
        )
        .await?;

        let layer_definitions = construct_layer_definitions(
            project_json.defs.layers,
            &tileset_definitions,
            load_context,
        )
        .await?;

        let enum_definitions = construct_enum_definitions(
            project_json.defs.enums,
            &tileset_definitions,
            &project_directory,
            load_context,
        )
        .await?;

        let entity_definitions = construct_entity_definitions(
            project_json.defs.entities,
            &tileset_definitions,
            load_context,
        )
        .await?;

        let worlds_json = if project_json.worlds.is_empty() {
            // if we're not a multi-world project, then we simply construct a
            // single [ldtk::World] and insert into an array. This saves us lots
            // of complexity when we transpose to our [crate::world::World]
            // object.
            IidMap::from([(
                iid,
                ldtk::World {
                    default_level_height: project_json.default_level_height.ok_or(
                        ldtk_import_error!("Missing default_level_height on single world project!"),
                    )?,
                    default_level_width: project_json.default_level_width.ok_or(
                        ldtk_import_error!("Missing default_level_width on single world project!"),
                    )?,
                    identifier: "World".to_string(),
                    iid: project_json.iid,
                    levels: project_json.levels,
                    world_grid_height: project_json.world_grid_height.ok_or(ldtk_import_error!(
                        "Missing world_grid_height on single world project!"
                    ))?,
                    world_grid_width: project_json.world_grid_width.ok_or(ldtk_import_error!(
                        "Missing world_grid_width on single world project!"
                    ))?,
                    world_layout: project_json.world_layout.clone(),
                },
            )])
        } else {
            project_json
                .worlds
                .into_iter()
                .map(|world_json| {
                    let iid = Iid::parse_str(&world_json.iid)?;
                    LdtkResult::Ok((iid, world_json))
                })
                .collect::<LdtkResult<_>>()?
        };

        let project_context = Arc::new(RwLock::new(ProjectContext {
            tileset_definitions,
            layer_definitions,
            enum_definitions,
            entity_definitions,
            external_levels: project_json.external_levels,
            project_directory,
        }));

        let worlds =
            construct_worlds_from_world_json(worlds_json, project_context, load_context).await?;

        Ok(Self {
            iid,
            ldtk_version,
            worlds,
        })
    }
}

impl LdtkAsset for Project {
    fn get_iid(&self) -> Iid {
        self.iid
    }
}

// impl LdtkAssetWithChildren<World> for Project {
//     fn get_children(&self) -> impl Iterator<Item = &Handle<World>> {
//         self.worlds.values()
//     }
// }
