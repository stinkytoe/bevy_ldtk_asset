use bevy::{prelude::*, utils::HashMap};

use crate::traits::NilToLoad;

#[derive(Clone, Component, Debug, Default, Reflect)]
pub enum EntitiesToLoad {
    None,
    ByIdentifiers(HashMap<String, NilToLoad>),
    ByIids(HashMap<String, NilToLoad>),
    #[default]
    All,
}
