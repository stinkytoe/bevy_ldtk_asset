//! A [Bevy](https://bevyengine.org) plugin for loading [LDtk](https://ldtk.io) project files!

#![warn(missing_docs)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod anchor;
mod color;
mod ldtk;
mod ldtk_path;
mod project_loader;
mod systems;

pub mod entity;
pub mod entity_definition;
pub mod enum_definition;
pub mod error;
pub mod field_instance;
pub mod iid;
pub mod layer;
pub mod layer_definition;
pub mod ldtk_asset_trait;
pub mod level;
pub mod plugin;
pub mod prelude;
pub mod project;
pub mod tile_instance;
pub mod tileset_definition;
pub mod tileset_rectangle;
pub mod uid;
pub mod world;

pub use error::LdtkError;

#[allow(missing_docs)]
pub mod result {
    pub type LdtkResult<T> = core::result::Result<T, crate::error::LdtkError>;
}

#[cfg(test)]
mod test;
