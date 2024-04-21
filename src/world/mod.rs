mod asset;
mod bundle;
mod component;

pub use asset::*;
pub use bundle::*;
pub use component::*;

use bevy::reflect::impl_reflect;

// re-exports
pub use crate::ldtk::WorldLayout;

#[cfg(feature = "enable_reflect")]
impl_reflect!(
    #[reflect(Debug)]
    #[type_path = "crate::ldtk"]
    pub enum WorldLayout {
        Free,
        GridVania,
        LinearHorizontal,
        LinearVertical,
    }
);
