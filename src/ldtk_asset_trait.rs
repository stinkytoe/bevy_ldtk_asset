use bevy_asset::Asset;

use crate::iid::Iid;

pub trait LdtkAsset: Asset {
    fn identifier(&self) -> &str;
    fn iid(&self) -> Iid;
}
