use bevy::asset::Asset;

use crate::iid::Iid;

pub trait LdtkAsset
where
    Self: Asset,
{
    fn iid(&self) -> Iid;
}
