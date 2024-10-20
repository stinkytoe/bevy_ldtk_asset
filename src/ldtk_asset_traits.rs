use bevy::asset::Asset;

use crate::iid::Iid;

pub trait LdtkAsset: Asset {}

pub trait HasIid
where
    Self: Asset,
{
    fn iid(&self) -> Iid;
}

pub trait HasIdentifier {
    fn identifier(&self) -> &str;
}

pub trait HasChildren {
    type Child: LdtkAsset;

    fn children(&self) -> impl Iterator<Item = &Self::Child>;
}
