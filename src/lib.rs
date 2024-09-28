// #![warn(missing_docs)]

mod anchor;
mod color;
mod ldtk;
mod ldtk_path;
mod project_loader;

pub mod entity;
pub mod enum_definition;
pub mod error;
pub mod field_instance;
pub mod iid;
pub mod layer;
pub mod ldtk_asset;
pub mod level;
pub mod plugin;
pub mod project;
pub mod tile_instance;
pub mod tileset_definition;
pub mod tileset_rectangle;
pub mod world;

pub mod prelude {
    pub use crate::entity::Entity;
    pub use crate::enum_definition::{EnumDefinition, EnumValueDefinition};
    pub use crate::error::Error;
    pub use crate::field_instance::{EntityRef, FieldInstance, FieldInstanceType};
    pub use crate::iid::{Iid, IidError, IidMap, IidSet};
    pub use crate::layer::{EntitiesLayer, Layer, LayerType, TilesLayer};
    pub use crate::ldtk_asset::LdtkAsset;
    pub use crate::level::{Level, LevelBackgroundPosition, Neighbour, NeighbourDir};
    pub use crate::plugin::BevyLdtkAssetPlugin;
    pub use crate::project::Project;
    pub use crate::tile_instance::TileInstance;
    pub use crate::tileset_definition::{EnumTagValue, TileCustomMetadata, TilesetDefinition};
    pub use crate::tileset_rectangle::TilesetRectangle;
    pub use crate::world::World;
}
