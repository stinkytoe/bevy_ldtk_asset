use bevy::asset::{Asset, AssetPath};

use crate::iid::Iid;

pub trait LdtkAsset
where
    Self: Asset,
{
    fn iid(&self) -> Iid;
    fn parent_path(&self) -> AssetPath;
    fn children_paths(&self) -> impl Iterator<Item = AssetPath>;
}
