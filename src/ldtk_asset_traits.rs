use bevy::asset::{Asset, Handle};

use crate::iid::Iid;

pub trait LdtkAsset: Asset + Sized {}

pub trait HasIid
where
    Self: Asset,
{
    fn iid(&self) -> Iid;
}

pub trait HasIdentifier {
    fn identifier(&self) -> &str;
}

pub trait HasChildren
where
    Self: Sized,
{
    type Child: LdtkAsset + Sized;

    fn children(&self) -> impl Iterator<Item = &Handle<Self::Child>>;
}
