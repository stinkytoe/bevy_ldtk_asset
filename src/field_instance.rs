use std::str::FromStr;

use bevy_asset::Handle;
use bevy_color::Color;
use bevy_math::I64Vec2;
use bevy_reflect::Reflect;
use bevy_utils::HashMap;

use crate::color::bevy_color_from_ldtk_string;
use crate::enum_definition::EnumDefinition;
use crate::iid::Iid;
use crate::ldtk;
use crate::ldtk_import_error;
use crate::tileset_definition::TilesetDefinition;
use crate::tileset_rectangle::TilesetRectangle;
use crate::uid::{Uid, UidMap};
use crate::Result;

#[derive(Debug, Reflect)]
pub struct EntityRef {
    pub entity_iid: Iid,
    pub layer_iid: Iid,
    pub level_iid: Iid,
    pub world_iid: Iid,
}

#[derive(Debug, Reflect)]
pub struct EnumValue {
    value: String,
    enum_definition: Handle<EnumDefinition>,
}

#[derive(Debug, Reflect)]
pub enum FieldInstanceType {
    ArrayInt(Vec<i64>),
    ArrayEnum(Vec<EnumValue>),
    ArrayMultilines(Vec<String>),
    ArrayPoint(Vec<I64Vec2>),
    ArrayTile(Vec<TilesetRectangle>),
    Bool(bool),
    Color(Color),
    EntityRef(Option<EntityRef>),
    Enum(Option<EnumValue>),
    FilePath(Option<String>),
    Float(Option<f64>),
    Int(Option<i64>),
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
        enum_definitions: &HashMap<String, Handle<EnumDefinition>>,
    ) -> Result<Self> {
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
            _ =>
            // try to parse as an enum
            {
                //Err(ldtk_import_error!(
                //    "Bad/Unknown Field Instance Type! given: {field_instance_type}"
                //))
                Self::parse_non_obvious_field_instance_type(
                    field_instance_type,
                    value,
                    enum_definitions,
                )
            }
        }
    }

    fn parse_non_obvious_field_instance_type(
        field_instance_type: &str,
        value: Option<&serde_json::Value>,
        enum_definitions: &HashMap<String, Handle<EnumDefinition>>,
    ) -> Result<Self> {
        // If we make it here, we should have one of four things:
        // * "LocalEnum.{Enum Group Name}"
        // * "ExternEnum.{Enum Group Name}"
        // * "Array<LocalEnum.{Enum Group Name}>"
        // * "Array<ExternEnum.{Enum Group Name}>"
        //
        // There is no difference between LocalEnum and ExternEnum, except the externalRelPath
        // field.
        //
        // The LocalEnum.{Enum Group Name} and ExternEnum.{Enum Group Name} will store a string in
        // their value field.
        //
        // The Array<...> variant stores an array of strings in its value field.

        let mut around_the_dot = field_instance_type.split('.');

        let (Some(lhs), Some(rhs), None) = (
            around_the_dot.next(),
            around_the_dot.next(),
            around_the_dot.next(),
        ) else {
            return Err(ldtk_import_error!(
                "Bad field instance type! {field_instance_type}"
            ));
        };

        let (enum_name, is_array) = if lhs.contains('<') {
            let mut around_the_carat = lhs.split('<');
            let (Some(array_lhs), Some(array_rhs), None) = (
                around_the_carat.next(),
                around_the_carat.next(),
                around_the_carat.next(),
            ) else {
                return Err(ldtk_import_error!(
                    "Bad field instance type! {field_instance_type}"
                ));
            };

            if array_lhs != "Array" {
                return Err(ldtk_import_error!(
                    "Bad field instance type! {field_instance_type}"
                ));
            }

            if !(array_rhs == "LocalEnum" || array_rhs == "ExternEnum") {
                return Err(ldtk_import_error!(
                    "Bad field instance type! {field_instance_type}"
                ));
            }

            // maybe redundant?
            if rhs.len() < 2 {
                return Err(ldtk_import_error!(
                    "Bad field instance type! {field_instance_type}"
                ));
            }

            if !rhs.ends_with('>') {
                return Err(ldtk_import_error!(
                    "Bad field instance type! {field_instance_type}"
                ));
            }

            (&rhs[..rhs.len() - 1], true)
        } else {
            if !(lhs == "LocalEnum" || lhs == "ExternEnum") {
                return Err(ldtk_import_error!(
                    "Bad field instance type! {field_instance_type}"
                ));
            }

            (rhs, false)
        };

        if is_array {
            let array_enum = value
                .and_then(|value| value.as_array())
                .map(|value| {
                    value
                        .iter()
                        .map(|value| {
                            serde_json::from_value::<String>(value.clone()).map_err(|e| e.into())
                        })
                        .collect::<Result<Vec<_>>>()
                })
                .transpose()?
                .map(|value| -> Result<Vec<EnumValue>> {
                    value
                        .into_iter()
                        .map(|value| -> Result<EnumValue> {
                            let enum_definition = enum_definitions
                                .get(enum_name)
                                .ok_or(ldtk_import_error!("bad enum identifier! {}", enum_name))?
                                .clone();

                            Ok(EnumValue {
                                value,
                                enum_definition,
                            })
                        })
                        .collect::<Result<Vec<_>>>()
                })
                .transpose()?
                .ok_or(ldtk_import_error!(
                    "Could not construct Vec<String> from ldtk input! given: {:?}",
                    value
                ))?;

            Ok(Self::ArrayEnum(array_enum))
        } else {
            let enum_value: Option<EnumValue> = value
                .map(|value| -> Result<String> {
                    serde_json::from_value::<String>(value.clone()).map_err(|e| e.into())
                })
                .transpose()?
                .map(|value| -> Result<EnumValue> {
                    let enum_definition = enum_definitions
                        .get(enum_name)
                        .ok_or(ldtk_import_error!("bad enum identifier! {}", enum_name))?
                        .clone();

                    Ok(EnumValue {
                        value,
                        enum_definition,
                    })
                })
                .transpose()?;

            Ok(Self::Enum(enum_value))
        }
    }
}

#[derive(Debug, Reflect)]
pub struct FieldInstance {
    pub tileset_rectangle: Option<TilesetRectangle>,
    pub field_instance_type: FieldInstanceType,
    pub def_uid: Uid,
}

impl FieldInstance {
    pub(crate) fn new(
        value: &ldtk::FieldInstance,
        tileset_definitions: &UidMap<Handle<TilesetDefinition>>,
        enum_definitions: &HashMap<String, Handle<EnumDefinition>>,
    ) -> Result<Self> {
        let tileset_rectangle = value
            .tile
            .as_ref()
            .map(|value| TilesetRectangle::new(value, tileset_definitions))
            .transpose()?;
        let field_instance_type = FieldInstanceType::new(
            &value.field_instance_type,
            value.value.as_ref(),
            tileset_definitions,
            enum_definitions,
        )?;
        let def_uid = value.def_uid;

        Ok(Self {
            tileset_rectangle,
            field_instance_type,
            def_uid,
        })
    }
}
