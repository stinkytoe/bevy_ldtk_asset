//! The Bevy LDtk asset loader!

#![warn(missing_docs)]

mod assets;
mod bundles;
#[allow(missing_docs)]
mod ldtk;
mod plugin;
mod structs;
mod systems;

/// Add `use bevy_ldtk_asset::prelude::*` to import the common interface
pub mod prelude {
    pub use crate::assets::level::LevelAsset;
    pub use crate::assets::project::ProjectAsset;
    pub use crate::assets::world::WorldAsset;
    pub use crate::bundles::WorldBundle;
    pub use crate::ldtk::WorldLayout;
    pub use crate::plugin::BevyLdtkAssetPlugin;
    pub use crate::structs::LoadParameters;
}
