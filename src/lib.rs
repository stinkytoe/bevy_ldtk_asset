//! The Bevy LDtk asset loader!

#![warn(missing_docs)]

mod assets;
mod bundles;
mod interface;
#[allow(missing_docs)]
mod ldtk;
mod plugin;
mod structs;
mod systems;
mod traits;
mod util;

/// Add `use bevy_ldtk_asset::prelude::*` to import the common interface
pub mod prelude {
    pub use crate::assets::level::LevelAsset;
    pub use crate::assets::project::ProjectAsset;
    pub use crate::assets::world::WorldAsset;
    pub use crate::bundles::LevelBundle;
    pub use crate::bundles::WorldBundle;
    pub use crate::interface::levels_at_position;
    pub use crate::interface::LevelAtPositionQuery;
    pub use crate::plugin::BevyLdtkAssetPlugin;
    pub use crate::structs::LdtkEntity;
    pub use crate::structs::SpawnEntities;
    pub use crate::traits::HasIdentifier;

    //
    pub use crate::ldtk::LayerInstance;
    //
    pub use crate::ldtk::FieldInstance;
    //
    pub use crate::ldtk::NeighbourLevel;
    /// Described the layout of the level in a specific world.
    pub use crate::ldtk::WorldLayout;
}
