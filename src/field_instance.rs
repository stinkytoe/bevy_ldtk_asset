use std::str::FromStr;

use bevy::color::Color;
use bevy::math::I64Vec2;
use bevy::reflect::Reflect;

use crate::color::bevy_color_from_ldtk_string;
use crate::error::Error;
use crate::iid::Iid;
use crate::ldtk;
use crate::tileset_rectangle::TilesetRectangle;

#[derive(Debug, Reflect)]
pub struct EntityRef {
    pub entity_iid: Iid,
    pub layer_iid: Iid,
    pub level_iid: Iid,
    pub world_iid: Iid,
}

#[derive(Debug, Reflect)]
pub enum FieldInstanceType {
    ArrayInt(Vec<i64>),
    ArrayLocalEnumSomeEnum(Vec<String>),
    ArrayMultilines(Vec<String>),
    ArrayPoint(Vec<I64Vec2>),
    ArrayTile(Vec<TilesetRectangle>),
    Bool(bool),
    Color(Color),
    EntityRef(Option<EntityRef>),
    ExternEnumAnExternEnum(Option<String>),
    FilePath(Option<String>),
    Float(Option<f64>),
    Int(Option<i64>),
    LocalEnumSomeEnum(Option<String>),
    Multilines(Option<String>),
    Point(Option<I64Vec2>),
    String(Option<String>),
    Tile(Option<TilesetRectangle>),
}

macro_rules! field_instance_unwrap {
    ($value:expr, $as_type: ident, $field_instance_type:ident) => {
        $value
            .ok_or(Error::LdtkImportError(format!(
                "Value is None when trying to parse a Field Instance of type {}!",
                $field_instance_type
            )))?
            .$as_type()
            .ok_or(Error::LdtkImportError(format!(
                "Value could not be coerced into type {}!",
                stringify!($variant),
            )))?
    };
}

macro_rules! field_instance_option_unwrap {
    ($value:expr, $as_type:ident, $field_instance_type:ident) => {
        if let Some(value) = $value {
            Some(value.$as_type().ok_or(Error::LdtkImportError(format!(
                "Could not convert using {} for field instance of type {}!",
                stringify!($as_type),
                $field_instance_type
            )))?)
        } else {
            None
        }
    };
}

macro_rules! field_instance_map_unwrap {
    ($map:expr, $key:expr, $field_type:expr, $as_type: ident) => {
        $map.get($key)
            .ok_or(Error::LdtkImportError(format!(
                "Field {} not in object for field instance type {}",
                $key, $field_type
            )))?
            .$as_type()
            .ok_or(Error::LdtkImportError(format!(
                "Could not parse with {} for field {} in {}!",
                stringify!($as_type),
                $key,
                $field_type
            )))?
    };
}

impl FieldInstanceType {
    pub(crate) fn new(
        field_instance_type: &str,
        value: Option<&serde_json::Value>,
    ) -> Result<Self, Error> {
        match field_instance_type {
            "Array<Int>" => Ok(Self::ArrayInt(
                field_instance_unwrap!(value, as_array, field_instance_type)
                    .iter()
                    .map(|value| {
                        value.as_i64().ok_or(Error::LdtkImportError(
                            "Could not parse array item with as_i64!".to_string(),
                        ))
                    })
                    .collect::<Result<_, _>>()?,
            )),
            "Array<LocalEnum.SomeEnum>" => Ok(Self::ArrayLocalEnumSomeEnum(
                field_instance_unwrap!(value, as_array, field_instance_type)
                    .iter()
                    .map(|value| -> Result<_, Error> {
                        Ok(value
                            .as_str()
                            .ok_or(Error::LdtkImportError(
                                "Could not parse array item as str!".to_string(),
                            ))?
                            .to_string())
                    })
                    .collect::<Result<_, _>>()?,
            )),
            "Array<Multilines>" => Ok(Self::ArrayMultilines(
                field_instance_unwrap!(value, as_array, field_instance_type)
                    .iter()
                    .map(|value| -> Result<_, Error> {
                        Ok(value
                            .as_str()
                            .ok_or(Error::LdtkImportError(
                                "Could not parse array item as str!".to_string(),
                            ))?
                            .to_string())
                    })
                    .collect::<Result<_, _>>()?,
            )),
            "Array<Point>" => Ok(Self::ArrayPoint(
                field_instance_unwrap!(value, as_array, field_instance_type)
                    .iter()
                    .map(|value| -> Result<_, Error> {
                        let cx = field_instance_map_unwrap!(value, "cx", "Array<Point>", as_i64);
                        let cy = field_instance_map_unwrap!(value, "cy", "Array<Point>", as_i64);
                        Ok((cx, cy).into())
                    })
                    .collect::<Result<_, _>>()?,
            )),
            "Array<Tile>" => Ok(Self::ArrayTile(
                field_instance_unwrap!(value, as_array, field_instance_type)
                    .iter()
                    .map(|value| serde_json::from_value::<ldtk::TilesetRectangle>(value.clone()))
                    .map(|value| value.map(|tile| TilesetRectangle::new(&tile)))
                    .collect::<Result<_, _>>()?,
            )),
            "Bool" => Ok(Self::Bool(field_instance_unwrap!(
                value,
                as_bool,
                field_instance_type
            ))),
            "Color" => Ok(Self::Color(bevy_color_from_ldtk_string(
                field_instance_unwrap!(value, as_str, field_instance_type),
            )?)),
            "EntityRef" => Ok(Self::EntityRef(
                if let Some(map) =
                    field_instance_option_unwrap!(value, as_object, field_instance_type)
                {
                    let entity_iid = Iid::from_str(field_instance_map_unwrap!(
                        map,
                        "entityIid",
                        "EntityRef",
                        as_str
                    ))?;
                    let layer_iid = Iid::from_str(field_instance_map_unwrap!(
                        map,
                        "layerIid",
                        "EntityRef",
                        as_str
                    ))?;
                    let level_iid = Iid::from_str(field_instance_map_unwrap!(
                        map,
                        "levelIid",
                        "EntityRef",
                        as_str
                    ))?;
                    let world_iid = Iid::from_str(field_instance_map_unwrap!(
                        map,
                        "worldIid",
                        "EntityRef",
                        as_str
                    ))?;
                    Some(EntityRef {
                        entity_iid,
                        layer_iid,
                        level_iid,
                        world_iid,
                    })
                } else {
                    None
                },
            )),
            "ExternEnum.AnExternEnum" => Ok(Self::ExternEnumAnExternEnum(
                field_instance_option_unwrap!(value, as_str, field_instance_type)
                    .map(|x| x.to_string()),
            )),
            "FilePath" => Ok(Self::FilePath(
                field_instance_option_unwrap!(value, as_str, field_instance_type)
                    .map(|x| x.to_string()),
            )),
            "Float" => Ok(Self::Float(field_instance_option_unwrap!(
                value,
                as_f64,
                field_instance_type
            ))),
            "Int" => Ok(Self::Int(field_instance_option_unwrap!(
                value,
                as_i64,
                field_instance_type
            ))),
            "LocalEnum.SomeEnum" => Ok(Self::LocalEnumSomeEnum(
                field_instance_option_unwrap!(value, as_str, field_instance_type)
                    .map(|x| x.to_string()),
            )),
            "Point" => Ok(Self::Point(
                if let Some(map) =
                    field_instance_option_unwrap!(value, as_object, field_instance_type)
                {
                    let cx = field_instance_map_unwrap!(map, "cx", "Point", as_i64);
                    let cy = field_instance_map_unwrap!(map, "cy", "Point", as_i64);
                    Some((cx, cy).into())
                } else {
                    None
                },
            )),
            "Multilines" => Ok(Self::Multilines(
                field_instance_option_unwrap!(value, as_str, field_instance_type)
                    .map(|x| x.to_string()),
            )),
            "String" => Ok(Self::String(
                field_instance_option_unwrap!(value, as_str, field_instance_type)
                    .map(|x| x.to_string()),
            )),
            "Tile" => Ok(Self::Tile({
                match value {
                    Some(value) => Some(serde_json::from_value::<ldtk::TilesetRectangle>(
                        value.clone(),
                    )?),
                    None => None,
                }
                .as_ref()
                .map(TilesetRectangle::new)
            })),
            _ => Err(Error::LdtkImportError(format!(
                "Bad/Unknown Field Instance Type! given: {field_instance_type}"
            ))),
        }
    }
}

#[derive(Debug, Reflect)]
pub struct FieldInstance {
    pub identifier: String,
    pub tileset_rectangle: Option<TilesetRectangle>,
    pub field_instance_type: FieldInstanceType,
    pub def_uid: i64,
}

impl FieldInstance {
    pub(crate) fn new(value: &ldtk::FieldInstance) -> Result<Self, Error> {
        let identifier = value.identifier.clone();
        let tileset_rectangle = value.tile.as_ref().map(TilesetRectangle::new);
        let field_instance_type =
            FieldInstanceType::new(&value.field_instance_type, value.value.as_ref())?;
        let def_uid = value.def_uid;

        Ok(Self {
            identifier,
            tileset_rectangle,
            field_instance_type,
            def_uid,
        })
    }
}
