use std::str::FromStr;

use bevy_asset::Handle;
use bevy_color::Color;
use bevy_math::I64Vec2;
use bevy_reflect::Reflect;

use crate::color::bevy_color_from_ldtk_string;
use crate::iid::Iid;
use crate::ldtk_import_error;
use crate::tileset_definition::TilesetDefinition;
use crate::tileset_rectangle::TilesetRectangle;
use crate::uid::{Uid, UidMap};
use crate::{ldtk, Result};

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

macro_rules! field_instance_map_get {
    ($map:expr, $key:expr, $field_type:expr, $as_type: ident) => {
        $map.get($key)
            .ok_or(ldtk_import_error!(
                "Field {} not in object for field instance type {}",
                $key,
                $field_type
            ))?
            .$as_type()
            .ok_or(ldtk_import_error!(
                "Could not parse with {} for field {} in {}!",
                stringify!($as_type),
                $key,
                $field_type,
            ))?
    };
}

impl FieldInstanceType {
    pub(crate) fn new(
        field_instance_type: &str,
        value: Option<&serde_json::Value>,
        tileset_definitions: &UidMap<Handle<TilesetDefinition>>,
    ) -> crate::Result<Self> {
        match field_instance_type {
            "Array<Int>" => Ok(Self::ArrayInt(
                value
                    .and_then(|value| value.as_array())
                    .map(|value| {
                        value
                            .iter()
                            .map(|value| {
                                serde_json::from_value::<i64>(value.clone()).map_err(|e| e.into())
                            })
                            .collect::<Result<Vec<_>>>()
                    })
                    .transpose()?
                    .ok_or(ldtk_import_error!(
                        "Could not construct Vec<i64> from ldtk input! given: {:?}",
                        value
                    ))?,
            )),
            "Array<LocalEnum.SomeEnum>" => Ok(Self::ArrayLocalEnumSomeEnum(
                value
                    .and_then(|value| value.as_array())
                    .map(|value| {
                        value
                            .iter()
                            .map(|value| {
                                serde_json::from_value::<String>(value.clone())
                                    .map_err(|e| e.into())
                            })
                            .collect::<Result<Vec<_>>>()
                    })
                    .transpose()?
                    .ok_or(ldtk_import_error!(
                        "Could not construct Vec<String> from ldtk input! given: {:?}",
                        value
                    ))?,
            )),
            "Array<Multilines>" => Ok(Self::ArrayMultilines(
                value
                    .and_then(|value| value.as_array())
                    .map(|value| {
                        value
                            .iter()
                            .map(|value| {
                                serde_json::from_value::<String>(value.clone())
                                    .map_err(|e| e.into())
                            })
                            .collect::<Result<Vec<_>>>()
                    })
                    .transpose()?
                    .ok_or(ldtk_import_error!(
                        "Could not construct Vec<String> from ldtk input! given: {:?}",
                        value
                    ))?,
            )),
            "Array<Point>" => Ok(Self::ArrayPoint(
                value
                    .and_then(|value| value.as_array())
                    .map(|value| {
                        value
                            .iter()
                            .map(|value| {
                                let cx = field_instance_map_get!(value, "cx", "Point", as_i64);
                                let cy = field_instance_map_get!(value, "cy", "Point", as_i64);
                                Ok((cx, cy).into())
                            })
                            .collect::<Result<Vec<_>>>()
                    })
                    .transpose()?
                    .ok_or(ldtk_import_error!(
                        "Could not construct Vec<I64Vec2> from ldtk input! given: {:?}",
                        value
                    ))?,
            )),
            "Array<Tile>" => Ok(Self::ArrayTile(
                value
                    .and_then(|value| value.as_array())
                    .map(|value| {
                        value
                            .iter()
                            .map(|value| {
                                let value = serde_json::from_value::<ldtk::TilesetRectangle>(
                                    value.clone(),
                                )?;

                                TilesetRectangle::new(&value, tileset_definitions)
                            })
                            .collect::<Result<Vec<_>>>()
                    })
                    .transpose()?
                    .ok_or(ldtk_import_error!(
                        "Could not construct Vec<TilesetRectangle> from ldtk input! given: {:?}",
                        value
                    ))?,
            )),
            "Bool" => Ok(Self::Bool(
                value
                    .map(|value| serde_json::from_value::<bool>(value.clone()))
                    .transpose()?
                    .ok_or(ldtk_import_error!(
                        "Could not construct bevy color from ldtk input! given: {:?}",
                        value
                    ))?,
            )),
            "Color" => Ok(Self::Color(
                value
                    .and_then(|value| value.as_str())
                    .map(bevy_color_from_ldtk_string)
                    .transpose()?
                    .ok_or(ldtk_import_error!(
                        "Could not construct bevy color from ldtk input! given: {:?}",
                        value
                    ))?,
            )),
            "EntityRef" => Ok(Self::EntityRef(
                value
                    .map(|value| -> Result<EntityRef> {
                        let entity_iid = Iid::from_str(field_instance_map_get!(
                            value,
                            "entityIid",
                            "EntityRef",
                            as_str
                        ))?;
                        let layer_iid = Iid::from_str(field_instance_map_get!(
                            value,
                            "layerIid",
                            "EntityRef",
                            as_str
                        ))?;
                        let level_iid = Iid::from_str(field_instance_map_get!(
                            value,
                            "levelIid",
                            "EntityRef",
                            as_str
                        ))?;
                        let world_iid = Iid::from_str(field_instance_map_get!(
                            value,
                            "worldIid",
                            "EntityRef",
                            as_str
                        ))?;
                        Ok(EntityRef {
                            entity_iid,
                            layer_iid,
                            level_iid,
                            world_iid,
                        })
                    })
                    .transpose()?,
            )),
            "ExternEnum.AnExternEnum" => Ok(Self::ExternEnumAnExternEnum(
                value
                    .map(|value| serde_json::from_value::<String>(value.clone()))
                    .transpose()?,
            )),
            "FilePath" => Ok(Self::FilePath(
                value
                    .map(|value| serde_json::from_value::<String>(value.clone()))
                    .transpose()?,
            )),
            "Float" => Ok(Self::Float(
                value
                    .map(|value| serde_json::from_value::<f64>(value.clone()))
                    .transpose()?,
            )),
            "Int" => Ok(Self::Int(
                value
                    .map(|value| serde_json::from_value::<i64>(value.clone()))
                    .transpose()?,
            )),
            "LocalEnum.SomeEnum" => Ok(Self::LocalEnumSomeEnum(
                value
                    .map(|value| serde_json::from_value::<String>(value.clone()))
                    .transpose()?,
            )),
            "Point" => Ok(Self::Point(
                value
                    .map(|value| -> Result<(i64, i64)> {
                        Ok((
                            field_instance_map_get!(value, "cx", "Point", as_i64),
                            field_instance_map_get!(value, "cy", "Point", as_i64),
                        ))
                    })
                    .transpose()?
                    .map(|pair| pair.into()),
            )),
            "Multilines" => Ok(Self::Multilines(
                value
                    .map(|value| serde_json::from_value::<String>(value.clone()))
                    .transpose()?,
            )),
            "String" => Ok(Self::String(
                value
                    .map(|value| serde_json::from_value::<String>(value.clone()))
                    .transpose()?,
            )),
            "Tile" => Ok(Self::Tile(
                value
                    .map(|value| serde_json::from_value::<ldtk::TilesetRectangle>(value.clone()))
                    .transpose()?
                    .map(|value| TilesetRectangle::new(&value, tileset_definitions))
                    .transpose()?,
            )),
            _ => Err(ldtk_import_error!(
                "Bad/Unknown Field Instance Type! given: {field_instance_type}"
            )),
        }
    }
}

#[derive(Debug, Reflect)]
pub struct FieldInstance {
    pub identifier: String,
    pub tileset_rectangle: Option<TilesetRectangle>,
    pub field_instance_type: FieldInstanceType,
    pub def_uid: Uid,
}

impl FieldInstance {
    pub(crate) fn new(
        value: &ldtk::FieldInstance,
        tileset_definitions: &UidMap<Handle<TilesetDefinition>>,
    ) -> crate::Result<Self> {
        let identifier = value.identifier.clone();
        let tileset_rectangle = value
            .tile
            .as_ref()
            .map(|value| TilesetRectangle::new(value, tileset_definitions))
            .transpose()?;
        let field_instance_type = FieldInstanceType::new(
            &value.field_instance_type,
            value.value.as_ref(),
            tileset_definitions,
        )?;
        let def_uid = value.def_uid;

        Ok(Self {
            identifier,
            tileset_rectangle,
            field_instance_type,
            def_uid,
        })
    }
}
