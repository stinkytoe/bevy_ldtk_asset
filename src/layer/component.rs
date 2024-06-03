use bevy::prelude::*;

#[derive(Clone, Component, Debug, Default, Reflect)]
pub enum EntitiesToLoad {
    None,
    ByIdentifiers(Vec<String>),
    ByIids(Vec<String>),
    #[default]
    All,
}
