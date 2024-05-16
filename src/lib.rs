// #![warn(missing_docs)]

// mod entity;
mod field_instance;
mod layer;
mod ldtk;
mod level;
mod plugins;
mod project;
// mod system_params;
mod tileset_rectangle;
mod util;
mod world;

pub mod prelude {
    // pub use crate::entity::*;
    pub use crate::field_instance::*;
    pub use crate::layer::*;
    pub use crate::level::*;
    pub use crate::plugins::*;
    pub use crate::project::*;
    // pub use crate::system_params::*;
    pub use crate::tileset_rectangle::*;
    pub use crate::world::*;
}

pub mod entity {
    use bevy::asset::Asset;
    use bevy::prelude::*;

    use crate::project::ProjectAsset;

    #[derive(Component, Debug, Default, Reflect)]
    pub enum EntitiesToLoad {
        None,
        ByIdentifiers(Vec<String>),
        ByIids(Vec<String>),
        #[default]
        All,
    }

    #[derive(Asset, Debug, Reflect)]
    pub struct EntityAsset {
        pub(crate) project: Handle<ProjectAsset>,
        pub(crate) iid: String,
    }

    #[derive(Component, Debug, Default, Reflect)]
    pub struct EntityComponent {}

    #[derive(Bundle, Debug, Default)]
    pub struct EntityBundle {
        pub(crate) world: Handle<EntityAsset>,
        pub(crate) spatial: SpatialBundle,
    }
}
