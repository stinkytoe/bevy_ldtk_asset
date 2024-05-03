use bevy::prelude::*;
use bevy::utils::thiserror;
use thiserror::Error;

use crate::field_instance::FieldInstance;
use crate::field_instance::FieldInstanceValueError;
use crate::tileset_rectangle::TilesetRectangle;
use crate::util::bevy_color_from_ldtk;
use crate::util::ColorParseError;

use crate::ldtk;

#[derive(Debug, Error)]
pub enum EntityComponentError {
    #[error("ColorParseError {0}")]
    ColorParseError(#[from] ColorParseError),
    #[error("WorldCoordMixedOptionError")]
    WorldCoordMixedOptionError,
    #[error("FieldInstanceValueError {0}")]
    FieldInstanceValueError(#[from] FieldInstanceValueError),
}

#[derive(Component, Debug)]
#[cfg_attr(feature = "enable_reflect", derive(Reflect))]
pub struct EntityComponent {
    grid: IVec2,
    identifier: String,
    pivot: Vec2,
    smart_color: Color,
    tags: Vec<String>,
    tile: Option<TilesetRectangle>,
    world_location: Option<Vec2>,
    def_uid: i64,
    field_instances: Vec<FieldInstance>,
    size: Vec2,
    iid: String,
    location: Vec2,
}

impl EntityComponent {
    pub fn grid(&self) -> IVec2 {
        self.grid
    }

    pub fn identifier(&self) -> &str {
        self.identifier.as_ref()
    }

    pub fn pivot(&self) -> Vec2 {
        self.pivot
    }

    pub fn smart_color(&self) -> Color {
        self.smart_color
    }

    pub fn tags(&self) -> &[String] {
        self.tags.as_ref()
    }

    pub fn tile(&self) -> Option<&TilesetRectangle> {
        self.tile.as_ref()
    }

    pub fn world_location(&self) -> Option<Vec2> {
        self.world_location
    }

    pub fn def_uid(&self) -> i64 {
        self.def_uid
    }

    pub fn field_instances(&self) -> &[FieldInstance] {
        self.field_instances.as_ref()
    }

    pub fn size(&self) -> Vec2 {
        self.size
    }

    pub fn iid(&self) -> &str {
        self.iid.as_ref()
    }

    pub fn location(&self) -> Vec2 {
        self.location
    }
}

impl TryFrom<&ldtk::EntityInstance> for EntityComponent {
    type Error = EntityComponentError;

    fn try_from(value: &ldtk::EntityInstance) -> Result<Self, Self::Error> {
        Ok(Self {
            grid: (value.grid[0] as i32, value.grid[1] as i32).into(),
            identifier: value.identifier.clone(),
            pivot: (value.pivot[0] as f32, value.pivot[1] as f32).into(),
            smart_color: bevy_color_from_ldtk(&value.smart_color)?,
            tags: value.tags.clone(),
            tile: value
                .tile
                .as_ref()
                .map(|tileset_rectangle| tileset_rectangle.into()),
            world_location: match (value.world_x, value.world_y) {
                (None, None) => None,
                (Some(world_x), Some(world_y)) => Some((world_x as f32, world_y as f32).into()),
                (None, Some(_)) | (Some(_), None) => {
                    return Err(EntityComponentError::WorldCoordMixedOptionError)
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
            location: (value.px[0] as f32, value.px[1] as f32).into(),
        })
    }
}

#[derive(Debug, Default)]
pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, _app: &mut App) {
        #[cfg(feature = "enable_reflect")]
        {
            _app //
                .register_type::<EntityComponent>();
        }
    }
}
