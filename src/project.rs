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
    //pub levels: IidMap<Handle<Level>>,
    //pub layers: IidMap<Handle<Layer>>,
    //pub entities: IidMap<Handle<Entity>>,
    //
    //pub parent_map: IidMap<Iid>,
    //pub children_map: IidMap<Vec<Iid>>,
    //
    //pub tilesets: HashMap<i64, Handle<Image>>,
    //
    //pub tileset_definitions: HashMap<i64, TilesetDefinition>,
    //pub enum_definitions: HashMap<i64, EnumDefinition>,
}

impl HasChildren for Project {
    type Child = World;

    fn children(&self) -> impl Iterator<Item = &Handle<Self::Child>> {
        self.worlds.values()
    }
}
