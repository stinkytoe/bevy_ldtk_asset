//! A field instance is a field value attached to certain assets.
//!
//! A vector of field instances can be attached to any of:
//! * [crate::entity::Entity]
//! * [crate::entity_definition::EntityDefinition]
//! * [crate::level::Level]
//!
//! See [FieldInstance](https://ldtk.io/json/#ldtk-FieldInstanceJson) for a full description.

use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

use bevy_asset::Handle;
use bevy_color::Color;
use bevy_math::I64Vec2;
use bevy_platform::collections::HashMap;
use bevy_reflect::Reflect;

use crate::LdtkResult;
use crate::color::bevy_color_from_ldtk_string;
use crate::enum_definition::EnumDefinition;
use crate::iid::Iid;
use crate::ldtk;
use crate::ldtk_import_error;
use crate::ldtk_path::ldtk_path_to_bevy_path;
use crate::tileset_definition::TilesetDefinition;
use crate::tileset_rectangle::TilesetRectangle;
use crate::uid::{Uid, UidMap};

/// The internal value of a field instance of type [FieldInstanceType::EntityRef]
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Reflect)]
pub struct EntityRef {
    pub entity_iid: Iid,
    pub layer_iid: Iid,
    pub level_iid: Iid,
    pub world_iid: Iid,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Reflect)]
pub struct EnumValue {
    value: String,
    enum_definition: Handle<EnumDefinition>,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Reflect)]
pub enum FieldInstanceType {
    ArrayInt(Vec<i64>),
    ArrayEnum(Vec<EnumValue>),
    ArrayString(Vec<String>),
    ArrayPoint(Vec<I64Vec2>),
    ArrayTile(Vec<TilesetRectangle>),
    Bool(bool),
    Color(Color),
    EntityRef(EntityRef),
    Enum(EnumValue),
    FilePath(PathBuf),
    Float(f64),
    Int(i64),
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
        base_directory: &Path,
        tileset_definitions: &UidMap<Handle<TilesetDefinition>>,
        enum_definitions: &HashMap<String, Handle<EnumDefinition>>,
    ) -> LdtkResult<Self> {
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
                    .collect::<LdtkResult<Vec<_>>>()?,
            )),
            "Array<String>" => Ok(Self::ArrayString(
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
                    .collect::<LdtkResult<Vec<_>>>()?,
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
                    .collect::<LdtkResult<Vec<_>>>()?,
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
                    .collect::<LdtkResult<Vec<_>>>()?,
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
            "FilePath" => Ok(Self::FilePath(
                ldtk_path_to_bevy_path(
                    base_directory,
                    serde_json::from_value::<String>(value.clone())?,
                )
                .to_path_buf(),
            )),
            "Float" => Ok(Self::Float(serde_json::from_value::<f64>(value.clone())?)),
            "Int" => Ok(Self::Int(serde_json::from_value::<i64>(value.clone())?)),
            "Point" => Ok(Self::Point(
                (
                    field_instance_map_get!(value, "cx", "Point", as_i64),
                    field_instance_map_get!(value, "cy", "Point", as_i64),
                )
                    .into(),
            )),
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
    ) -> LdtkResult<Self> {
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
                .collect::<LdtkResult<Vec<_>>>()?;

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
        }
    }
}

/// An individual field instance value.
///
/// Typically, this will be stored in a collection in either an
/// [crate::entity::Entity] or a [crate::level::Level], and be associated with that particular
/// asset.
///
/// Note: Optional field instance values which have not been given a value are not exported by
/// this plugin. If the field is required by LDtk, but not supplied, then the LDtk editor will
/// flash a red border around the entity warning the user to fill in the field. If the field is
/// allowed to be null, then we simply will not export it.
#[allow(missing_docs)]
#[derive(Clone, Debug, Reflect)]
pub struct FieldInstance {
    pub tileset_rectangle: Option<TilesetRectangle>,
    pub field_instance_type: FieldInstanceType,
    pub def_uid: Uid,
}

impl FieldInstance {
    pub(crate) fn new(
        value: &ldtk::FieldInstance,
        base_directory: &Path,
        tileset_definitions: &UidMap<Handle<TilesetDefinition>>,
        enum_definitions: &HashMap<String, Handle<EnumDefinition>>,
    ) -> LdtkResult<Self> {
        let tileset_rectangle = value
            .tile
            .as_ref()
            .map(|value| TilesetRectangle::new(value, tileset_definitions))
            .transpose()?;
        let field_instance_type = FieldInstanceType::new(
            &value.field_instance_type,
            value.value.as_ref(),
            base_directory,
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

macro_rules! is_type {
    ($self:expr, $field_instance_type:path) => {
        matches!($self.field_instance_type, $field_instance_type(_))
    };
}

#[allow(missing_docs)]
impl FieldInstance {
    pub fn is_array_int(&self) -> bool {
        is_type!(self, FieldInstanceType::ArrayInt)
    }

    pub fn is_array_enum(&self) -> bool {
        is_type!(self, FieldInstanceType::ArrayEnum)
    }

    pub fn is_array_string(&self) -> bool {
        is_type!(self, FieldInstanceType::ArrayString)
    }

    pub fn is_array_point(&self) -> bool {
        is_type!(self, FieldInstanceType::ArrayPoint)
    }

    pub fn is_array_tile(&self) -> bool {
        is_type!(self, FieldInstanceType::ArrayTile)
    }

    pub fn is_bool(&self) -> bool {
        is_type!(self, FieldInstanceType::Bool)
    }

    pub fn is_color(&self) -> bool {
        is_type!(self, FieldInstanceType::Color)
    }

    pub fn is_entity_ref(&self) -> bool {
        is_type!(self, FieldInstanceType::EntityRef)
    }

    pub fn is_enum(&self) -> bool {
        is_type!(self, FieldInstanceType::Enum)
    }

    pub fn is_file_path(&self) -> bool {
        is_type!(self, FieldInstanceType::FilePath)
    }

    pub fn is_float(&self) -> bool {
        is_type!(self, FieldInstanceType::Float)
    }

    pub fn is_int(&self) -> bool {
        is_type!(self, FieldInstanceType::Int)
    }

    pub fn is_point(&self) -> bool {
        is_type!(self, FieldInstanceType::Point)
    }

    pub fn is_string(&self) -> bool {
        is_type!(self, FieldInstanceType::String)
    }

    pub fn is_tile(&self) -> bool {
        is_type!(self, FieldInstanceType::Tile)
    }
}

macro_rules! get_by_type {
    ( $fn_name:ident, $layer_instance_type:path, $ret:path) => {
        pub fn $fn_name(&self) -> Option<&$ret> {
            if let $layer_instance_type(inner) = &self.field_instance_type {
                Some(inner)
            } else {
                None
            }
        }
    };
}

#[rustfmt::skip::macros(get_by_type)]
#[allow(missing_docs)]
impl FieldInstance {
    get_by_type!(get_array_int, FieldInstanceType::ArrayInt, Vec<i64>);
    get_by_type!(get_array_enum, FieldInstanceType::ArrayEnum, Vec<EnumValue>);
    get_by_type!(get_array_string, FieldInstanceType::ArrayString, Vec<String>);
    get_by_type!(get_array_point, FieldInstanceType::ArrayPoint, Vec<I64Vec2>);
    get_by_type!(get_array_tile, FieldInstanceType::ArrayTile, Vec<TilesetRectangle>);
    get_by_type!(get_bool, FieldInstanceType::Bool, bool);
    get_by_type!(get_color, FieldInstanceType::Color, Color);
    get_by_type!(get_array_entity_ref,FieldInstanceType::EntityRef, EntityRef);
    get_by_type!(get_enum, FieldInstanceType::Enum, EnumValue);
    get_by_type!(get_file_path, FieldInstanceType::FilePath, PathBuf);
    get_by_type!(get_float, FieldInstanceType::Float, f64);
    get_by_type!(get_int, FieldInstanceType::Int, i64);
    get_by_type!(get_point, FieldInstanceType::Point, I64Vec2);
    get_by_type!(get_string, FieldInstanceType::String, String);
    get_by_type!(get_tile, FieldInstanceType::Tile, TilesetRectangle);
}
