use bevy::asset::Asset;
use bevy::asset::Handle;
use bevy::reflect::Reflect;

use crate::iid::Iid;
use crate::iid::IidMap;
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
