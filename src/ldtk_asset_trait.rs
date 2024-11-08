use bevy_asset::Asset;
use bevy_math::Vec3;

use crate::iid::Iid;

pub trait LdtkAsset: Asset {
    fn get_identifier(&self) -> &str;
    fn get_iid(&self) -> Iid;
    fn get_translation(&self) -> Vec3;
}
