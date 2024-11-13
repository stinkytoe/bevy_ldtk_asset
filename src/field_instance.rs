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
    EntityRef(EntityRef),
    Enum(EnumValue),
    FilePath(String),
    Float(f64),
    Int(i64),
    Multilines(String),
    Point(I64Vec2),
    String(String),
    Tile(TilesetRectangle),
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
        let value = value.ok_or(ldtk_import_error!("Field instance value is None!"))?;
        match field_instance_type {
            "Array<Int>" => Ok(Self::ArrayInt(
                value
                    .as_array()
                    .ok_or(ldtk_import_error!(
                        "Field Instance with type Array<Int>, value not an array! {:?}",
                        value
                    ))?
                    .iter()
                    .map(|value| serde_json::from_value::<i64>(value.clone()).map_err(|e| e.into()))
                    .collect::<Result<Vec<_>>>()?,
            )),
            "Array<Multilines>" => Ok(Self::ArrayMultilines(
                value
                    .as_array()
                    .ok_or(ldtk_import_error!(
                        "Field Instance with type Array<Multilines>, value not an array! {:?}",
                        value
                    ))?
                    .iter()
                    .map(|value| {
                        serde_json::from_value::<String>(value.clone()).map_err(|e| e.into())
                    })
                    .collect::<Result<Vec<_>>>()?,
            )),
            "Array<Point>" => Ok(Self::ArrayPoint(
                value
                    .as_array()
                    .ok_or(ldtk_import_error!(
                        "Field Instance with type Array<Point>, value not an array! {:?}",
                        value
                    ))?
                    .iter()
                    .map(|value| {
                        let cx = field_instance_map_get!(value, "cx", "Point", as_i64);
                        let cy = field_instance_map_get!(value, "cy", "Point", as_i64);
                        Ok((cx, cy).into())
                    })
                    .collect::<Result<Vec<_>>>()?,
            )),
            "Array<Tile>" => Ok(Self::ArrayTile(
                value
                    .as_array()
                    .ok_or(ldtk_import_error!(
                        "Field Instance with type Array<Tile>, value not an array! {:?}",
                        value
                    ))?
                    .iter()
                    .map(|value| {
                        let value =
                            serde_json::from_value::<ldtk::TilesetRectangle>(value.clone())?;
                        TilesetRectangle::new(&value, tileset_definitions)
                    })
                    .collect::<Result<Vec<_>>>()?,
            )),
            "Bool" => Ok(Self::Bool(serde_json::from_value::<bool>(value.clone())?)),
            "Color" => Ok(Self::Color({
                let value = serde_json::from_value::<String>(value.clone())?;
                bevy_color_from_ldtk_string(&value)?
            })),
            "EntityRef" => Ok(Self::EntityRef({
                let entity_iid = field_instance_map_get!(value, "entityIid", "EntityRef", as_str);
                let entity_iid = Iid::from_str(entity_iid)?;

                let layer_iid = field_instance_map_get!(value, "layerIid", "EntityRef", as_str);
                let layer_iid = Iid::from_str(layer_iid)?;

                let level_iid = field_instance_map_get!(value, "levelIid", "EntityRef", as_str);
                let level_iid = Iid::from_str(level_iid)?;

                let world_iid = field_instance_map_get!(value, "worldIid", "EntityRef", as_str);
                let world_iid = Iid::from_str(world_iid)?;

                EntityRef {
                    entity_iid,
                    layer_iid,
                    level_iid,
                    world_iid,
                }
            })),
            // TODO: Should we refactor this to a Bevy asset path with
            // ldtk_path_to_bevy_path?
            "FilePath" => Ok(Self::FilePath(serde_json::from_value::<String>(
                value.clone(),
            )?)),
            "Float" => Ok(Self::Float(serde_json::from_value::<f64>(value.clone())?)),
            "Int" => Ok(Self::Int(serde_json::from_value::<i64>(value.clone())?)),
            "Point" => Ok(Self::Point(
                (
                    field_instance_map_get!(value, "cx", "Point", as_i64),
                    field_instance_map_get!(value, "cy", "Point", as_i64),
                )
                    .into(),
            )),
            "Multilines" => Ok(Self::Multilines(serde_json::from_value::<String>(
                value.clone(),
            )?)),
            "String" => Ok(Self::String(serde_json::from_value::<String>(
                value.clone(),
            )?)),
            "Tile" => Ok(Self::Tile({
                let value = serde_json::from_value::<ldtk::TilesetRectangle>(value.clone())?;
                TilesetRectangle::new(&value, tileset_definitions)?
            })),
            _ => {
                // try to parse as an enum
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
        value: &serde_json::Value,
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
                .as_array()
                .ok_or(ldtk_import_error!(
                    "Field Instance with type Array<*Enum.*>, value not an array!"
                ))?
                .iter()
                .map(|value| {
                    let value = serde_json::from_value::<String>(value.clone())?;
                    let enum_definition = enum_definitions
                        .get(enum_name)
                        .ok_or(ldtk_import_error!("bad enum identifier! {}", enum_name))?
                        .clone();

                    Ok(EnumValue {
                        value,
                        enum_definition,
                    })
                })
                .collect::<Result<Vec<_>>>()?;

            Ok(Self::ArrayEnum(array_enum))
        } else {
            let value = serde_json::from_value::<String>(value.clone())?;
            let enum_definition = enum_definitions
                .get(enum_name)
                .ok_or(ldtk_import_error!("bad enum identifier! {}", enum_name))?
                .clone();

            Ok(Self::Enum(EnumValue {
                value,
                enum_definition,
            }))
            //let enum_value: Option<EnumValue> = value
            //    .map(|value| -> Result<String> {
            //        serde_json::from_value::<String>(value.clone()).map_err(|e| e.into())
            //    })
            //    .transpose()?
            //    .map(|value| -> Result<EnumValue> {
            //        let enum_definition = enum_definitions
            //            .get(enum_name)
            //            .ok_or(ldtk_import_error!("bad enum identifier! {}", enum_name))?
            //            .clone();
            //
            //        Ok(EnumValue {
            //            value,
            //            enum_definition,
            //        })
            //    })
            //    .transpose()?;
            //
            //Ok(Self::Enum(enum_value))
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
