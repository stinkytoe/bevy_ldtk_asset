use bevy::asset::Asset;
use bevy::asset::Handle;
use bevy::reflect::Reflect;

use crate::iid::Iid;
use crate::iid::IidMap;
use crate::ldtk_asset_traits::HasChildren;
use crate::tileset_definition::TilesetDefinition;
use crate::uid::UidMap;
use crate::world::World;

#[derive(Asset, Debug, Reflect)]
pub struct Project {
    pub iid: Iid,
    pub json_version: String,

    pub tileset_definitions: UidMap<Handle<TilesetDefinition>>,

    pub worlds: IidMap<Handle<World>>,
}

impl HasChildren for Project {
    type Child = World;

    fn children(&self) -> impl Iterator<Item = &Handle<Self::Child>> {
        self.worlds.values()
    }
}
