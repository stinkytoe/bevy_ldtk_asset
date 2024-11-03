use bevy_asset::{Asset, Handle};

use crate::iid::Iid;

pub trait LdtkAsset: Asset {
    fn identifier(&self) -> &str;
    fn iid(&self) -> Iid;
}

pub trait HasChildren
where
    Self: Sized,
{
    type Child: LdtkAsset;

    fn children(&self) -> impl Iterator<Item = &Handle<Self::Child>>;
}
