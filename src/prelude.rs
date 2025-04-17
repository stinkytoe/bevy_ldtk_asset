//! A convenient place to re-export the most used types.

// The main LDtk types.
pub use crate::entity::EntityInstance;
pub use crate::layer::LayerInstance;
pub use crate::level::{Level, LevelBackground, Neighbour, NeighbourDir};
pub use crate::project::Project;
pub use crate::world::{World, WorldLayout};

// Definitions
pub use crate::entity_definition::{EntityDefinition, TileRenderMode};
pub use crate::enum_definition::{EnumDefinition, EnumValueDefinition};
pub use crate::layer_definition::{
    IntGridValue, IntGridValuesGroup, IntGridValuesGroups, LayerDefinition, LayerDefinitionType,
};
pub use crate::tileset_definition::{EnumTagValue, TileCustomMetadata, TilesetDefinition};

// Others
pub use crate::field_instance::{EntityRef, EnumValue, FieldInstance, FieldInstanceType};
pub use crate::tile_instance::TileInstance;
pub use crate::tileset_rectangle::TilesetRectangle;

// Iids/Uids
pub use crate::iid::{Iid, IidMap, IidSet, iid};
pub use crate::uid::{Uid, UidMap, UidSet};

// Traits
pub use crate::ldtk_asset_trait::LdtkAsset;
pub use crate::ldtk_asset_trait::LdtkAssetWithChildren;
pub use crate::ldtk_asset_trait::LdtkAssetWithFieldInstances;
pub use crate::ldtk_asset_trait::LdtkAssetWithTags;
