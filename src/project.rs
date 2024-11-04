use bevy_asset::Asset;
use bevy_asset::Handle;
use bevy_reflect::Reflect;

use crate::iid::Iid;
use crate::iid::IidMap;
use crate::world::World;

#[derive(Asset, Debug, Reflect)]
pub struct Project {
    pub iid: Iid,
    pub json_version: String,

    pub worlds: IidMap<Handle<World>>,
}
