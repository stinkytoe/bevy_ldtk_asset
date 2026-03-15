#![allow(missing_docs)]
use bevy_asset::{Asset, Handle};

// use crate::field_instance::FieldInstance;
use crate::{field_instance::FieldInstance, iid::Iid};

/// Trait representing the assets which would exist in an LDtk project world, such as:
/// - [crate::project::Project]
/// - [crate::world::World]
/// - [crate::level::Level],
/// - [crate::layer::LayerInstance]
/// - [crate::entity::EntityInstance].
#[allow(missing_docs)]
pub trait LdtkAsset: Asset {
    fn get_identifier(&self) -> &str;
    fn get_iid(&self) -> Iid;
}

/// Trait representing assets which have children in the LDtk world hierarchy.
///
/// * [crate::project::Project]s contain [crate::world::World]s
/// * [crate::world::World]s contain [crate::level::Level]s
/// * [crate::level::Level]s contain [crate::layer::LayerInstance]s
/// * [crate::layer::LayerInstance]s contain [crate::entity::EntityInstance]s
pub trait LdtkAssetWithChildren<ChildAsset: LdtkAsset>: LdtkAsset {
    #[allow(missing_docs)]
    fn get_children(&self) -> impl Iterator<Item = &Handle<ChildAsset>>;
}

/// Trait representing assets which have field instances.
#[allow(missing_docs)]
pub trait LdtkAssetWithFieldInstances: LdtkAsset {
    fn get_field_instance(&self, identifier: &str) -> Option<&FieldInstance>;
}

/// Trait representing assets with a `tags` field. The following types use this:
/// - [crate::entity::EntityInstance]
/// - [crate::tileset_definition::TilesetDefinition]
/// - [crate::enum_definition::EnumDefinition]
#[allow(missing_docs)]
pub trait LdtkAssetWithTags: Asset {
    fn get_tags(&self) -> &[String];

    fn has_tag(&self, tag: &str) -> bool;
}
