use bevy::prelude::*;
use bevy::sprite::Anchor;
use thiserror::Error;

use crate::field_instance::FieldInstance;
use crate::field_instance::FieldInstanceValueParseError;
use crate::ldtk;
use crate::project::ProjectAsset;
use crate::tileset_rectangle::TilesetRectangle;
use crate::util::bevy_anchor_from_ldtk;
use crate::util::bevy_color_from_ldtk;
use crate::util::AnchorIntoError;
use crate::util::ColorParseError;

#[derive(Debug, Error)]
pub enum NewEntityAssetError {
    #[error(transparent)]
    AnchorIntoError(#[from] AnchorIntoError),
    #[error(transparent)]
    ColorParseError(#[from] ColorParseError),
    #[error(transparent)]
    FieldInstanceValueError(#[from] FieldInstanceValueParseError),
    #[error("One world coord is Some(...) and the other is None!")]
    WorldCoordMixedOption,
}

#[derive(Asset, Debug, Reflect)]
pub struct EntityAsset {
    grid: IVec2,
    identifier: String,
    anchor: Anchor,
    smart_color: Color,
    tags: Vec<String>,
    tile: Option<TilesetRectangle>,
    world_location: Option<Vec2>,
    def_uid: i64,
    field_instances: Vec<FieldInstance>,
    size: Vec2,
    iid: String,
    location: Vec2,
    #[reflect(ignore)]
    _project: Handle<ProjectAsset>,
}

impl EntityAsset {
    pub fn new(
        value: &ldtk::EntityInstance,
        project: Handle<ProjectAsset>,
    ) -> Result<Self, NewEntityAssetError> {
        Ok(Self {
            grid: (value.grid[0] as i32, value.grid[1] as i32).into(),
            identifier: value.identifier.clone(),
            anchor: bevy_anchor_from_ldtk(&value.pivot)?,
            smart_color: bevy_color_from_ldtk(&value.smart_color)?,
            tags: value.tags.clone(),
            tile: value.tile.as_ref().map(TilesetRectangle::from),
            world_location: match (value.world_x, value.world_y) {
                (None, None) => None,
                (Some(world_x), Some(world_y)) => Some((world_x as f32, world_y as f32).into()),
                (None, Some(_)) | (Some(_), None) => {
                    return Err(NewEntityAssetError::WorldCoordMixedOption)
                }
            },
            def_uid: value.def_uid,
            field_instances: value
                .field_instances
                .iter()
                .map(|field_instance| field_instance.try_into())
                .collect::<Result<_, _>>()?,
            size: (value.width as f32, value.height as f32).into(),
            iid: value.iid.clone(),
            location: (value.px[0] as f32, -value.px[1] as f32).into(),
            _project: project,
        })
    }
}
