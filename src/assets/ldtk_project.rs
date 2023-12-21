use crate::ldtk_json;
use bevy::prelude::*;

#[derive(Asset, Debug, TypePath)]
pub struct LdtkProject {
    pub value: ldtk_json::LdtkJson,
}
