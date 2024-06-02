use bevy::prelude::*;

#[derive(Component, Debug, PartialEq, Reflect)]
pub struct Iid {
    pub iid: String,
}

impl Iid {
    pub fn new(iid: String) -> Self {
        Self { iid }
    }
}

#[derive(Component, Debug, PartialEq, Reflect)]
pub struct Size {
    pub size: Vec2,
}

pub(crate) struct CommonComponentsPlugin;

impl Plugin for CommonComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Iid>();
    }
}
