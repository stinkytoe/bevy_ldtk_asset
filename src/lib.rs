//! The Bevy LDtk asset loader!

#![warn(missing_docs)]

mod assets;
mod bundles;
mod ldtk;
mod plugin;
mod resources;
mod systems;
mod util;

// An auto generated library of the LDtk project representation.
// Based on the manifest provided by [LDtk](https://ldtk.io) and generated using
// [Quicktype](https://quicktype.io/). Parsed using
// [serde_json](https://docs.rs/serde_json/latest/serde_json/).
#[allow(missing_docs)]
#[allow(rustdoc::bare_urls)]
#[allow(clippy::enum_variant_names)]
mod ldtk_json;

/// Include this for all of the public interface.
pub mod prelude {
    pub use crate::bundles::LdtkLevelBundle;
    pub use crate::ldtk::entity_instance::EntityInstance;
    pub use crate::ldtk::int_grid_value::IntGridValue;
    pub use crate::ldtk::layer_definition::LayerDefinition;
    pub use crate::ldtk::layer_instance::LayerInstance;
    pub use crate::ldtk::level_asset::LevelAsset;
    pub use crate::ldtk::level_component::LevelComponent;
    pub use crate::ldtk::project::Project;
    pub use crate::plugin::BevyLdtkAssetPlugin;
}
