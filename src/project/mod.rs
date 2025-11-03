//! The LDtk project top level representation!

mod construct_tileset_definitions;

use bevy_asset::Asset;
use bevy_asset::LoadContext;
use bevy_reflect::Reflect;

use crate::iid::Iid;
use crate::ldtk;
use crate::ldtk_asset_trait::LdtkAsset;
use crate::ldtk_import_error;
use crate::result::Result;

use construct_tileset_definitions::construct_tileset_definitions;

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
    //
    // The global collection of tileset definitions.
    // pub tileset_definitions: UidMap<Handle<TilesetDefinition>>,
    // The associated worlds in this project, indexed by their [Iid]s.
    //
    // This is the top level of the entire sub asset heirarchy.
    // pub worlds: IidMap<Handle<World>>,
}

impl Project {
    pub(crate) async fn new(
        project_json: ldtk::LdtkProject,
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self> {
        let iid: Iid = project_json.iid.try_into()?;

        let ldtk_version = project_json.json_version;

        let project_directory = load_context
            .path()
            .parent()
            .ok_or(ldtk_import_error!("Unable to get project_directory!"))?
            .to_path_buf();

        let _tileset_definitions = construct_tileset_definitions(
            &project_directory,
            project_json.defs.tilesets,
            load_context,
        )
        .await?;

        Ok(Self { iid, ldtk_version })
    }
}

impl Project {}

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
