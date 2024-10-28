// #![warn(missing_docs)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod anchor;
mod color;
mod ldtk;
mod ldtk_path;
mod project_loader;
mod systems;

pub mod entity;
pub mod enum_definition;
pub mod error;
pub mod field_instance;
pub mod iid;
pub mod label;
pub mod layer;
pub mod ldtk_asset_traits;
pub mod level;
pub mod plugin;
pub mod project;
pub mod tile_instance;
pub mod tileset_definition;
pub mod tileset_rectangle;
pub mod uid;
pub mod world;

pub use error::{Error, Result};

pub mod prelude {
    pub use crate::enum_definition::{EnumDefinition, EnumValueDefinition};
    pub use crate::field_instance::{EntityRef, FieldInstance, FieldInstanceType};
    pub use crate::iid::{Iid, IidError, IidMap, IidSet};
    pub use crate::layer::{EntitiesLayer, LayerType, TilesLayer};
    pub use crate::level::{LevelBackgroundPosition, Neighbour, NeighbourDir};
    pub use crate::plugin::BevyLdtkAssetPlugin;
    pub use crate::tile_instance::TileInstance;
    pub use crate::tileset_definition::{EnumTagValue, TileCustomMetadata, TilesetDefinition};
    pub use crate::tileset_rectangle::TilesetRectangle;
    pub use crate::uid::{Uid, UidMap, UidSet};

    pub mod ldtk_asset {
        pub use crate::entity::Entity;
        pub use crate::layer::Layer;
        pub use crate::level::Level;
        pub use crate::project::Project;
        pub use crate::world::World;

        pub use crate::ldtk_asset_traits::HasChildren;
        pub use crate::ldtk_asset_traits::HasIdentifier;
        pub use crate::ldtk_asset_traits::HasIid;
        pub use crate::ldtk_asset_traits::LdtkAsset;
    }
}
