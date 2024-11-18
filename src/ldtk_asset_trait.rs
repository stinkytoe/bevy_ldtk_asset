#![allow(missing_docs)]
use bevy_asset::{Asset, Handle};

use crate::field_instance::FieldInstance;
use crate::iid::Iid;

/// Trait representing the assets which would exist in an LDtk project world, such as
/// [crate::project::Project], [crate::world::World], [crate::level::Level], [crate::layer::Layer],
/// and [crate::entity::Entity].
#[allow(missing_docs)]
pub trait LdtkAsset: Asset {
    fn get_identifier(&self) -> &str;
    fn get_iid(&self) -> Iid;
}

/// Trait representing assets which have children in the LDtk world hierarchy.
///
/// * [crate::project::Project]s contain [crate::world::World]s
/// * [crate::world::World]s contain [crate::level::Level]s
/// * [crate::level::Level]s contain [crate::layer::Layer]s
/// * [crate::layer::Layer]s contain [crate::entity::Entity]s
pub trait LdtkAssetWithChildren<ChildAsset: LdtkAsset>: LdtkAsset {
    #[allow(missing_docs)]
    fn get_children(&self) -> impl Iterator<Item = &Handle<ChildAsset>>;
}

/// Trait representing assets which have field instances.
pub trait LdtkAssetWithFieldInstances: LdtkAsset {
    #[allow(missing_docs)]
    fn get_field_instance(&self, identifier: &str) -> Option<&FieldInstance>;
}
