use bevy::asset::Asset;
use bevy::asset::Handle;
use bevy::reflect::Reflect;

use crate::iid::Iid;
use crate::iid::IidMap;
use crate::ldtk_asset_traits::HasChildren;
use crate::world::World;

#[derive(Asset, Debug, Reflect)]
pub struct Project {
    pub iid: Iid,
    pub json_version: String,

    pub worlds: IidMap<Handle<World>>,
}

impl HasChildren for Project {
    type Child = World;

    fn children(&self) -> impl Iterator<Item = &Handle<Self::Child>> {
        self.worlds.values()
    }
}
