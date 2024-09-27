use bevy::asset::Asset;
use bevy::asset::Handle;
use bevy::reflect::Reflect;
use bevy::render::texture::Image;
use bevy::utils::HashMap;

use crate::entity::Entity;
use crate::iid::Iid;
use crate::iid::IidMap;
use crate::layer::Layer;
use crate::level::Level;
use crate::tileset_definition::TilesetDefinition;
use crate::world::World;

#[derive(Asset, Debug, Reflect)]
pub struct Project {
    pub iid: Iid,
    pub json_version: String,

    pub worlds: IidMap<Handle<World>>,
    pub levels: IidMap<Handle<Level>>,
    pub layers: IidMap<Handle<Layer>>,
    pub entities: IidMap<Handle<Entity>>,

    pub tileset_images: HashMap<String, Handle<Image>>,
    pub tileset_definitions: HashMap<i64, TilesetDefinition>,

    pub parent_map: IidMap<Iid>,
}
