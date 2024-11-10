use bevy_asset::Asset;
use bevy_asset::Handle;
use bevy_reflect::Reflect;

use crate::iid::Iid;
use crate::iid::IidMap;
use crate::ldtk_asset_trait::LdtkAsset;
use crate::ldtk_asset_trait::LdtkAssetWithChildren;
use crate::world::World;

#[derive(Asset, Debug, Reflect)]
pub struct Project {
    pub iid: Iid,
    pub json_version: String,
    pub path: String,

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
