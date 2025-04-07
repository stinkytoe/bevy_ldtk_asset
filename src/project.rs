//! The LDtk project top level representation!
use bevy_asset::Asset;
use bevy_asset::Handle;
use bevy_reflect::Reflect;

use crate::iid::Iid;
use crate::iid::IidMap;
use crate::ldtk_asset_trait::LdtkAsset;
use crate::ldtk_asset_trait::LdtkAssetWithChildren;
use crate::world::World;

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
    pub json_version: String,
    /// The Bevy asset relative path where this project was loaded from.
    pub path: String,

    /// The associated worlds in this project, indexed by their [Iid]s.
    ///
    /// This is the top level of the entire sub asset heirarchy.
    pub worlds: IidMap<Handle<World>>,
}

impl LdtkAsset for Project {
    fn get_identifier(&self) -> &str {
        &self.path
    }

    fn get_iid(&self) -> Iid {
        self.iid
    }
}

impl LdtkAssetWithChildren<World> for Project {
    fn get_children(&self) -> impl Iterator<Item = &Handle<World>> {
        self.worlds.values()
    }
}
