//! The Bevy LDtk asset loader!

#![warn(missing_docs)]

mod assets;
mod bundles;
mod components;
mod plugin;
mod resources;
mod systems;
mod util;

/// An auto generated library of the LDtk project representation.
/// Based on the manifest provided by [LDtk](https://ldtk.io) and generated using
/// [Quicktype](https://quicktype.io/). Parsed using
/// [serde_json](https://docs.rs/serde_json/latest/serde_json/).
#[allow(missing_docs)]
#[allow(rustdoc::bare_urls)]
pub mod ldtk_json;

/// Include this for all of the public interface.
pub mod prelude {
    pub use crate::assets::ldtk_level::LdtkLevel;
    pub use crate::assets::ldtk_project::LdtkProject;
    pub use crate::bundles::LdtkLevelBundle;
    pub use crate::components::*;
    pub use crate::plugin::BevyLdtkAssetPlugin;
}
