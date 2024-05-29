use bevy::prelude::*;

#[derive(Component, Debug, PartialEq, Reflect)]
pub struct Iid {
    pub iid: String,
}

pub(crate) struct CommonComponentsPlugin;

impl Plugin for CommonComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Iid>();
    }
}

#[derive(Component, Debug, PartialEq, Reflect)]
pub struct Size {
    pub size: Vec2,
}
