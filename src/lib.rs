//! The Bevy LDtk asset loader!

#![warn(missing_docs)]

mod assets;
mod bundles;
mod components;
mod plugin;
mod resources;
mod systems;
mod util;

// #[doc(hidden)]
#[allow(missing_docs)]
#[allow(rustdoc::bare_urls)]
pub mod ldtk_json;

/// Include this for all of the public interface
pub mod prelude {
    pub use crate::assets::ldtk_level::LdtkLevel;
    pub use crate::assets::ldtk_project::LdtkProject;
    pub use crate::bundles::LdtkLevelBundle;
    pub use crate::components::*;
    pub use crate::plugin::BevyLdtkAssetPlugin;
}
