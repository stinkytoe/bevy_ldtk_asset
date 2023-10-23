use crate::ldtk_json;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use serde::Deserialize;

#[derive(Asset, TypePath, Debug, Deserialize)]
pub(crate) struct LdtkLevel {
    pub(crate) value: ldtk_json::Level,
}
