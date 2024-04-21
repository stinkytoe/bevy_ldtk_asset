use bevy::reflect::impl_reflect;

pub use crate::ldtk::ldtk_json_1_5_3::WorldLayout;
impl_reflect!(
    #[reflect(Debug)]
    #[type_path = "crate::ldtk"]
    pub enum WorldLayout {
        Free,
        GridVania,
        LinearHorizontal,
        LinearVertical,
    }
);
