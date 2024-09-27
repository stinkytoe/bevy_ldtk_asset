use std::str::FromStr;

use bevy::asset::Asset;
use bevy::color::Color;
use bevy::math::{Rect, Vec2};
use bevy::reflect::Reflect;

use crate::color::bevy_color_from_ldtk_string;
use crate::error::Error;
use crate::field_instance::FieldInstance;
use crate::iid::Iid;
use crate::ldtk;
use crate::neighbour::Neighbour;

#[derive(Debug, Reflect)]
pub struct LevelBackgroundPosition {
    pub crop_rect: Rect,
    pub scale: Vec2,
    pub corner: Vec2,
}

impl LevelBackgroundPosition {
    pub(crate) fn new(value: &ldtk::LevelBackgroundPosition) -> Result<Self, Error> {
        let crop_rect = (value.crop_rect.len() == 4)
            .then(|| {
                let p0 = (value.crop_rect[0] as f32, value.crop_rect[1] as f32).into();
                let size = Vec2::new(value.crop_rect[2] as f32, value.crop_rect[3] as f32);
                let p1 = p0 + size;
                Rect::from_corners(p0, p1)
            })
            .ok_or(Error::LdtkImportError(format!(
                "Bad value for crop_rect! given: {:?}",
                value.crop_rect
            )))?;
        let scale = (value.scale.len() == 2)
            .then(|| (value.scale[0] as f32, value.scale[1] as f32).into())
            .ok_or(Error::LdtkImportError(format!(
                "Bad value for scale! given: {:?}",
                value.crop_rect
            )))?;
        let corner = (value.top_left_px.len() == 2)
            .then(|| (value.top_left_px[0] as f32, value.top_left_px[1] as f32).into())
            .ok_or(Error::LdtkImportError(format!(
                "Bad value for corner! given: {:?}",
                value.crop_rect
            )))?;

        Ok(Self {
            crop_rect,
            scale,
            corner,
        })
    }
}

#[derive(Asset, Debug, Reflect)]
pub struct Level {
    pub bg_color: Color,
    pub bg_pos: Option<LevelBackgroundPosition>,
    pub neighbours: Vec<Neighbour>,
    pub bg_rel_path: Option<String>,
    pub field_instances: Vec<FieldInstance>,
    pub identifier: String,
    pub iid: Iid,
    pub size: Vec2,
    pub uid: i64, // TODO: do we need this?
    pub world_depth: i64,
    pub location: Vec2,
}

impl Level {
    pub(crate) fn new(value: &ldtk::Level) -> Result<Self, Error> {
        let bg_color = bevy_color_from_ldtk_string(&value.bg_color)?;
        let bg_pos: Option<LevelBackgroundPosition> = match value.bg_pos.as_ref() {
            Some(bg_pos) => Some(LevelBackgroundPosition::new(bg_pos)?),
            None => None,
        };
        let neighbours = value
            .neighbours
            .iter()
            .map(Neighbour::new)
            .collect::<Result<_, _>>()?;
        let bg_rel_path = value.bg_rel_path.clone();
        let field_instances = value
            .field_instances
            .iter()
            .map(FieldInstance::new)
            .collect::<Result<_, _>>()?;
        let identifier = value.identifier.clone();
        let iid = Iid::from_str(&value.iid)?;
        let size = (value.px_wid as f32, value.px_hei as f32).into();
        let uid = value.uid;
        let world_depth = value.world_depth;
        let location = (value.world_x as f32, -value.world_y as f32).into();

        Ok(Level {
            bg_color,
            bg_pos,
            neighbours,
            bg_rel_path,
            field_instances,
            identifier,
            iid,
            //layer_instances,
            size,
            uid,
            world_depth,
            location,
        })
    }
}
