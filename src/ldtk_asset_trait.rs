use bevy_asset::{Asset, Handle};

use crate::field_instance::FieldInstance;
use crate::iid::Iid;

pub trait LdtkAsset: Asset {
    fn get_identifier(&self) -> &str;
    fn get_iid(&self) -> Iid;
}

pub trait LdtkAssetWithChildren<ChildAsset: LdtkAsset>: LdtkAsset {
    fn get_children(&self) -> impl Iterator<Item = &Handle<ChildAsset>>;
}

pub trait LdtkAssetWithFieldInstances: LdtkAsset {
    fn get_field_instance(&self, identifier: &str) -> Option<&FieldInstance>;
}
