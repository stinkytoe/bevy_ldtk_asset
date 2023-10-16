// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::LDTK-JSON;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: LDTK-JSON = serde_json::from_str(&json).unwrap();
// }

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// This file is a JSON schema of files created by LDtk level editor (https://ldtk.io).
///
/// This is the root of any Project JSON file. It contains:  - the project settings, - an
/// array of levels, - a group of definitions (that can probably be safely ignored for most
/// users).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LdtkJson {
    /// This object is not actually used by LDtk. It ONLY exists to force explicit references to
    /// all types, to make sure QuickType finds them and integrate all of them. Otherwise,
    /// Quicktype will drop types that are not explicitely used.
    #[serde(rename = "__FORCED_REFS")]
    pub(crate) forced_refs: Option<ForcedRefs>,

    /// LDtk application build identifier.<br/>  This is only used to identify the LDtk version
    /// that generated this particular project file, which can be useful for specific bug fixing.
    /// Note that the build identifier is just the date of the release, so it's not unique to
    /// each user (one single global ID per LDtk public release), and as a result, completely
    /// anonymous.
    pub(crate) app_build_id: f64,

    /// Number of backup files to keep, if the `backupOnSave` is TRUE
    pub(crate) backup_limit: i64,

    /// If TRUE, an extra copy of the project will be created in a sub folder, when saving.
    pub(crate) backup_on_save: bool,

    /// Target relative path to store backup files
    pub(crate) backup_rel_path: Option<String>,

    /// Project background color
    pub(crate) bg_color: String,

    /// An array of command lines that can be ran manually by the user
    pub(crate) custom_commands: Vec<LdtkCustomCommand>,

    /// Default height for new entities
    pub(crate) default_entity_height: Option<i64>,

    /// Default width for new entities
    pub(crate) default_entity_width: Option<i64>,

    /// Default grid size for new layers
    pub(crate) default_grid_size: i64,

    /// Default background color of levels
    pub(crate) default_level_bg_color: String,

    /// **WARNING**: this field will move to the `worlds` array after the "multi-worlds" update.
    /// It will then be `null`. You can enable the Multi-worlds advanced project option to enable
    /// the change immediately.<br/><br/>  Default new level height
    pub(crate) default_level_height: Option<i64>,

    /// **WARNING**: this field will move to the `worlds` array after the "multi-worlds" update.
    /// It will then be `null`. You can enable the Multi-worlds advanced project option to enable
    /// the change immediately.<br/><br/>  Default new level width
    pub(crate) default_level_width: Option<i64>,

    /// Default X pivot (0 to 1) for new entities
    pub(crate) default_pivot_x: f64,

    /// Default Y pivot (0 to 1) for new entities
    pub(crate) default_pivot_y: f64,

    /// A structure containing all the definitions of this project
    pub(crate) defs: Definitions,

    /// If the project isn't in MultiWorlds mode, this is the IID of the internal "dummy" World.
    pub(crate) dummy_world_iid: String,

    /// If TRUE, the exported PNGs will include the level background (color or image).
    pub(crate) export_level_bg: bool,

    /// **WARNING**: this deprecated value is no longer exported since version 0.9.3  Replaced
    /// by: `imageExportMode`
    pub(crate) export_png: Option<bool>,

    /// If TRUE, a Tiled compatible file will also be generated along with the LDtk JSON file
    /// (default is FALSE)
    pub(crate) export_tiled: bool,

    /// If TRUE, one file will be saved for the project (incl. all its definitions) and one file
    /// in a sub-folder for each level.
    pub(crate) external_levels: bool,

    /// An array containing various advanced flags (ie. options or other states). Possible
    /// values: `DiscardPreCsvIntGrid`, `ExportPreCsvIntGridFormat`, `IgnoreBackupSuggest`,
    /// `PrependIndexToLevelFileNames`, `MultiWorlds`, `UseMultilinesType`
    pub(crate) flags: Vec<Flag>,

    /// Naming convention for Identifiers (first-letter uppercase, full uppercase etc.) Possible
    /// values: `Capitalize`, `Uppercase`, `Lowercase`, `Free`
    pub(crate) identifier_style: IdentifierStyle,

    /// Unique project identifier
    pub(crate) iid: String,

    /// "Image export" option when saving project. Possible values: `None`, `OneImagePerLayer`,
    /// `OneImagePerLevel`, `LayersAndLevels`
    pub(crate) image_export_mode: ImageExportMode,

    /// File format version
    pub(crate) json_version: String,

    /// The default naming convention for level identifiers.
    pub(crate) level_name_pattern: String,

    /// All levels. The order of this array is only relevant in `LinearHorizontal` and
    /// `linearVertical` world layouts (see `worldLayout` value).<br/>  Otherwise, you should
    /// refer to the `worldX`,`worldY` coordinates of each Level.
    pub(crate) levels: Vec<Level>,

    /// If TRUE, the Json is partially minified (no indentation, nor line breaks, default is
    /// FALSE)
    pub(crate) minify_json: bool,

    /// Next Unique integer ID available
    pub(crate) next_uid: i64,

    /// File naming pattern for exported PNGs
    pub(crate) png_file_pattern: Option<String>,

    /// If TRUE, a very simplified will be generated on saving, for quicker & easier engine
    /// integration.
    pub(crate) simplified_export: bool,

    /// All instances of entities that have their `exportToToc` flag enabled are listed in this
    /// array.
    pub(crate) toc: Vec<LdtkTableOfContentEntry>,

    /// This optional description is used by LDtk Samples to show up some informations and
    /// instructions.
    pub(crate) tutorial_desc: Option<String>,

    /// **WARNING**: this field will move to the `worlds` array after the "multi-worlds" update.
    /// It will then be `null`. You can enable the Multi-worlds advanced project option to enable
    /// the change immediately.<br/><br/>  Height of the world grid in pixels.
    pub(crate) world_grid_height: Option<i64>,

    /// **WARNING**: this field will move to the `worlds` array after the "multi-worlds" update.
    /// It will then be `null`. You can enable the Multi-worlds advanced project option to enable
    /// the change immediately.<br/><br/>  Width of the world grid in pixels.
    pub(crate) world_grid_width: Option<i64>,

    /// **WARNING**: this field will move to the `worlds` array after the "multi-worlds" update.
    /// It will then be `null`. You can enable the Multi-worlds advanced project option to enable
    /// the change immediately.<br/><br/>  An enum that describes how levels are organized in
    /// this project (ie. linearly or in a 2D space). Possible values: &lt;`null`&gt;, `Free`,
    /// `GridVania`, `LinearHorizontal`, `LinearVertical`
    pub(crate) world_layout: Option<WorldLayout>,

    /// This array will be empty, unless you enable the Multi-Worlds in the project advanced
    /// settings.<br/><br/> - in current version, a LDtk project file can only contain a single
    /// world with multiple levels in it. In this case, levels and world layout related settings
    /// are stored in the root of the JSON.<br/> - with "Multi-worlds" enabled, there will be a
    /// `worlds` array in root, each world containing levels and layout settings. Basically, it's
    /// pretty much only about moving the `levels` array to the `worlds` array, along with world
    /// layout related values (eg. `worldGridWidth` etc).<br/><br/>If you want to start
    /// supporting this future update easily, please refer to this documentation:
    /// https://github.com/deepnight/ldtk/issues/231
    pub(crate) worlds: Vec<World>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LdtkCustomCommand {
    pub(crate) command: String,

    /// Possible values: `Manual`, `AfterLoad`, `BeforeSave`, `AfterSave`
    pub(crate) when: When,
}

/// Possible values: `Manual`, `AfterLoad`, `BeforeSave`, `AfterSave`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum When {
    #[serde(rename = "AfterLoad")]
    AfterLoad,

    #[serde(rename = "AfterSave")]
    AfterSave,

    #[serde(rename = "BeforeSave")]
    BeforeSave,

    Manual,
}

/// If you're writing your own LDtk importer, you should probably just ignore *most* stuff in
/// the `defs` section, as it contains data that are mostly important to the editor. To keep
/// you away from the `defs` section and avoid some unnecessary JSON parsing, important data
/// from definitions is often duplicated in fields prefixed with a double underscore (eg.
/// `__identifier` or `__type`).  The 2 only definition types you might need here are
/// **Tilesets** and **Enums**.
///
/// A structure containing all the definitions of this project
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Definitions {
    /// All entities definitions, including their custom fields
    pub(crate) entities: Vec<EntityDefinition>,

    /// All internal enums
    pub(crate) enums: Vec<EnumDefinition>,

    /// Note: external enums are exactly the same as `enums`, except they have a `relPath` to
    /// point to an external source file.
    pub(crate) external_enums: Vec<EnumDefinition>,

    /// All layer definitions
    pub(crate) layers: Vec<LayerDefinition>,

    /// All custom fields available to all levels.
    pub(crate) level_fields: Vec<FieldDefinition>,

    /// All tilesets
    pub(crate) tilesets: Vec<TilesetDefinition>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityDefinition {
    /// Base entity color
    pub(crate) color: String,

    /// User defined documentation for this element to provide help/tips to level designers.
    pub(crate) doc: Option<String>,

    /// If enabled, all instances of this entity will be listed in the project "Table of content"
    /// object.
    pub(crate) export_to_toc: bool,

    /// Array of field definitions
    pub(crate) field_defs: Vec<FieldDefinition>,

    pub(crate) fill_opacity: f64,

    /// Pixel height
    pub(crate) height: i64,

    pub(crate) hollow: bool,

    /// User defined unique identifier
    pub(crate) identifier: String,

    /// Only applies to entities resizable on both X/Y. If TRUE, the entity instance width/height
    /// will keep the same aspect ratio as the definition.
    pub(crate) keep_aspect_ratio: bool,

    /// Possible values: `DiscardOldOnes`, `PreventAdding`, `MoveLastOne`
    pub(crate) limit_behavior: LimitBehavior,

    /// If TRUE, the maxCount is a "per world" limit, if FALSE, it's a "per level". Possible
    /// values: `PerLayer`, `PerLevel`, `PerWorld`
    pub(crate) limit_scope: LimitScope,

    pub(crate) line_opacity: f64,

    /// Max instances count
    pub(crate) max_count: i64,

    /// Max pixel height (only applies if the entity is resizable on Y)
    pub(crate) max_height: Option<i64>,

    /// Max pixel width (only applies if the entity is resizable on X)
    pub(crate) max_width: Option<i64>,

    /// Min pixel height (only applies if the entity is resizable on Y)
    pub(crate) min_height: Option<i64>,

    /// Min pixel width (only applies if the entity is resizable on X)
    pub(crate) min_width: Option<i64>,

    /// An array of 4 dimensions for the up/right/down/left borders (in this order) when using
    /// 9-slice mode for `tileRenderMode`.<br/>  If the tileRenderMode is not NineSlice, then
    /// this array is empty.<br/>  See: https://en.wikipedia.org/wiki/9-slice_scaling
    pub(crate) nine_slice_borders: Vec<i64>,

    /// Pivot X coordinate (from 0 to 1.0)
    pub(crate) pivot_x: f64,

    /// Pivot Y coordinate (from 0 to 1.0)
    pub(crate) pivot_y: f64,

    /// Possible values: `Rectangle`, `Ellipse`, `Tile`, `Cross`
    pub(crate) render_mode: RenderMode,

    /// If TRUE, the entity instances will be resizable horizontally
    pub(crate) resizable_x: bool,

    /// If TRUE, the entity instances will be resizable vertically
    pub(crate) resizable_y: bool,

    /// Display entity name in editor
    pub(crate) show_name: bool,

    /// An array of strings that classifies this entity
    pub(crate) tags: Vec<String>,

    /// **WARNING**: this deprecated value is no longer exported since version 1.2.0  Replaced
    /// by: `tileRect`
    pub(crate) tile_id: Option<i64>,

    pub(crate) tile_opacity: f64,

    /// An object representing a rectangle from an existing Tileset
    pub(crate) tile_rect: Option<TilesetRectangle>,

    /// An enum describing how the the Entity tile is rendered inside the Entity bounds. Possible
    /// values: `Cover`, `FitInside`, `Repeat`, `Stretch`, `FullSizeCropped`,
    /// `FullSizeUncropped`, `NineSlice`
    pub(crate) tile_render_mode: TileRenderMode,

    /// Tileset ID used for optional tile display
    pub(crate) tileset_id: Option<i64>,

    /// Unique Int identifier
    pub(crate) uid: i64,

    /// This tile overrides the one defined in `tileRect` in the UI
    pub(crate) ui_tile_rect: Option<TilesetRectangle>,

    /// Pixel width
    pub(crate) width: i64,
}

/// This section is mostly only intended for the LDtk editor app itself. You can safely
/// ignore it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldDefinition {
    /// Human readable value type. Possible values: `Int, Float, String, Bool, Color,
    /// ExternEnum.XXX, LocalEnum.XXX, Point, FilePath`.<br/>  If the field is an array, this
    /// field will look like `Array<...>` (eg. `Array<Int>`, `Array<Point>` etc.)<br/>  NOTE: if
    /// you enable the advanced option **Use Multilines type**, you will have "*Multilines*"
    /// instead of "*String*" when relevant.
    #[serde(rename = "__type")]
    pub(crate) field_definition_type: String,

    /// Optional list of accepted file extensions for FilePath value type. Includes the dot:
    /// `.ext`
    pub(crate) accept_file_types: Option<Vec<String>>,

    /// Possible values: `Any`, `OnlySame`, `OnlyTags`, `OnlySpecificEntity`
    pub(crate) allowed_refs: AllowedRefs,

    pub(crate) allowed_refs_entity_uid: Option<i64>,

    pub(crate) allowed_ref_tags: Vec<String>,

    pub(crate) allow_out_of_level_ref: bool,

    /// Array max length
    pub(crate) array_max_length: Option<i64>,

    /// Array min length
    pub(crate) array_min_length: Option<i64>,

    pub(crate) auto_chain_ref: bool,

    /// TRUE if the value can be null. For arrays, TRUE means it can contain null values
    /// (exception: array of Points can't have null values).
    pub(crate) can_be_null: bool,

    /// Default value if selected value is null or invalid.
    pub(crate) default_override: Option<serde_json::Value>,

    /// User defined documentation for this field to provide help/tips to level designers about
    /// accepted values.
    pub(crate) doc: Option<String>,

    pub(crate) editor_always_show: bool,

    pub(crate) editor_cut_long_values: bool,

    pub(crate) editor_display_color: Option<String>,

    /// Possible values: `Hidden`, `ValueOnly`, `NameAndValue`, `EntityTile`, `LevelTile`,
    /// `Points`, `PointStar`, `PointPath`, `PointPathLoop`, `RadiusPx`, `RadiusGrid`,
    /// `ArrayCountWithLabel`, `ArrayCountNoLabel`, `RefLinkBetweenPivots`,
    /// `RefLinkBetweenCenters`
    pub(crate) editor_display_mode: EditorDisplayMode,

    /// Possible values: `Above`, `Center`, `Beneath`
    pub(crate) editor_display_pos: EditorDisplayPos,

    pub(crate) editor_display_scale: f64,

    /// Possible values: `ZigZag`, `StraightArrow`, `CurvedArrow`, `ArrowsLine`, `DashedLine`
    pub(crate) editor_link_style: EditorLinkStyle,

    pub(crate) editor_show_in_world: bool,

    pub(crate) editor_text_prefix: Option<String>,

    pub(crate) editor_text_suffix: Option<String>,

    /// User defined unique identifier
    pub(crate) identifier: String,

    /// TRUE if the value is an array of multiple values
    pub(crate) is_array: bool,

    /// Max limit for value, if applicable
    pub(crate) max: Option<f64>,

    /// Min limit for value, if applicable
    pub(crate) min: Option<f64>,

    /// Optional regular expression that needs to be matched to accept values. Expected format:
    /// `/some_reg_ex/g`, with optional "i" flag.
    pub(crate) regex: Option<String>,

    pub(crate) symmetrical_ref: bool,

    /// Possible values: &lt;`null`&gt;, `LangPython`, `LangRuby`, `LangJS`, `LangLua`, `LangC`,
    /// `LangHaxe`, `LangMarkdown`, `LangJson`, `LangXml`, `LangLog`
    pub(crate) text_language_mode: Option<TextLanguageMode>,

    /// UID of the tileset used for a Tile
    pub(crate) tileset_uid: Option<i64>,

    /// Internal enum representing the possible field types. Possible values: F_Int, F_Float,
    /// F_String, F_Text, F_Bool, F_Color, F_Enum(...), F_Point, F_Path, F_EntityRef, F_Tile
    #[serde(rename = "type")]
    pub(crate) purple_type: String,

    /// Unique Int identifier
    pub(crate) uid: i64,

    /// If TRUE, the color associated with this field will override the Entity or Level default
    /// color in the editor UI. For Enum fields, this would be the color associated to their
    /// values.
    pub(crate) use_for_smart_color: bool,
}

/// Possible values: `Any`, `OnlySame`, `OnlyTags`, `OnlySpecificEntity`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AllowedRefs {
    Any,

    #[serde(rename = "OnlySame")]
    OnlySame,

    #[serde(rename = "OnlySpecificEntity")]
    OnlySpecificEntity,

    #[serde(rename = "OnlyTags")]
    OnlyTags,
}

/// Possible values: `Hidden`, `ValueOnly`, `NameAndValue`, `EntityTile`, `LevelTile`,
/// `Points`, `PointStar`, `PointPath`, `PointPathLoop`, `RadiusPx`, `RadiusGrid`,
/// `ArrayCountWithLabel`, `ArrayCountNoLabel`, `RefLinkBetweenPivots`,
/// `RefLinkBetweenCenters`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EditorDisplayMode {
    #[serde(rename = "ArrayCountNoLabel")]
    ArrayCountNoLabel,

    #[serde(rename = "ArrayCountWithLabel")]
    ArrayCountWithLabel,

    #[serde(rename = "EntityTile")]
    EntityTile,

    Hidden,

    #[serde(rename = "LevelTile")]
    LevelTile,

    #[serde(rename = "NameAndValue")]
    NameAndValue,

    #[serde(rename = "PointPath")]
    PointPath,

    #[serde(rename = "PointPathLoop")]
    PointPathLoop,

    #[serde(rename = "PointStar")]
    PointStar,

    Points,

    #[serde(rename = "RadiusGrid")]
    RadiusGrid,

    #[serde(rename = "RadiusPx")]
    RadiusPx,

    #[serde(rename = "RefLinkBetweenCenters")]
    RefLinkBetweenCenters,

    #[serde(rename = "RefLinkBetweenPivots")]
    RefLinkBetweenPivots,

    #[serde(rename = "ValueOnly")]
    ValueOnly,
}

/// Possible values: `Above`, `Center`, `Beneath`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EditorDisplayPos {
    Above,

    Beneath,

    Center,
}

/// Possible values: `ZigZag`, `StraightArrow`, `CurvedArrow`, `ArrowsLine`, `DashedLine`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EditorLinkStyle {
    #[serde(rename = "ArrowsLine")]
    ArrowsLine,

    #[serde(rename = "CurvedArrow")]
    CurvedArrow,

    #[serde(rename = "DashedLine")]
    DashedLine,

    #[serde(rename = "StraightArrow")]
    StraightArrow,

    #[serde(rename = "ZigZag")]
    ZigZag,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TextLanguageMode {
    #[serde(rename = "LangC")]
    LangC,

    #[serde(rename = "LangHaxe")]
    LangHaxe,

    #[serde(rename = "LangJS")]
    LangJs,

    #[serde(rename = "LangJson")]
    LangJson,

    #[serde(rename = "LangLog")]
    LangLog,

    #[serde(rename = "LangLua")]
    LangLua,

    #[serde(rename = "LangMarkdown")]
    LangMarkdown,

    #[serde(rename = "LangPython")]
    LangPython,

    #[serde(rename = "LangRuby")]
    LangRuby,

    #[serde(rename = "LangXml")]
    LangXml,
}

/// Possible values: `DiscardOldOnes`, `PreventAdding`, `MoveLastOne`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LimitBehavior {
    #[serde(rename = "DiscardOldOnes")]
    DiscardOldOnes,

    #[serde(rename = "MoveLastOne")]
    MoveLastOne,

    #[serde(rename = "PreventAdding")]
    PreventAdding,
}

/// If TRUE, the maxCount is a "per world" limit, if FALSE, it's a "per level". Possible
/// values: `PerLayer`, `PerLevel`, `PerWorld`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LimitScope {
    #[serde(rename = "PerLayer")]
    PerLayer,

    #[serde(rename = "PerLevel")]
    PerLevel,

    #[serde(rename = "PerWorld")]
    PerWorld,
}

/// Possible values: `Rectangle`, `Ellipse`, `Tile`, `Cross`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RenderMode {
    Cross,

    Ellipse,

    Rectangle,

    Tile,
}

/// This object represents a custom sub rectangle in a Tileset image.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TilesetRectangle {
    /// Height in pixels
    pub(crate) h: i64,

    /// UID of the tileset
    pub(crate) tileset_uid: i64,

    /// Width in pixels
    pub(crate) w: i64,

    /// X pixels coordinate of the top-left corner in the Tileset image
    pub(crate) x: i64,

    /// Y pixels coordinate of the top-left corner in the Tileset image
    pub(crate) y: i64,
}

/// An enum describing how the the Entity tile is rendered inside the Entity bounds. Possible
/// values: `Cover`, `FitInside`, `Repeat`, `Stretch`, `FullSizeCropped`,
/// `FullSizeUncropped`, `NineSlice`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TileRenderMode {
    Cover,

    #[serde(rename = "FitInside")]
    FitInside,

    #[serde(rename = "FullSizeCropped")]
    FullSizeCropped,

    #[serde(rename = "FullSizeUncropped")]
    FullSizeUncropped,

    #[serde(rename = "NineSlice")]
    NineSlice,

    Repeat,

    Stretch,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnumDefinition {
    pub(crate) external_file_checksum: Option<String>,

    /// Relative path to the external file providing this Enum
    pub(crate) external_rel_path: Option<String>,

    /// Tileset UID if provided
    pub(crate) icon_tileset_uid: Option<i64>,

    /// User defined unique identifier
    pub(crate) identifier: String,

    /// An array of user-defined tags to organize the Enums
    pub(crate) tags: Vec<String>,

    /// Unique Int identifier
    pub(crate) uid: i64,

    /// All possible enum values, with their optional Tile infos.
    pub(crate) values: Vec<EnumValueDefinition>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnumValueDefinition {
    /// **WARNING**: this deprecated value is no longer exported since version 1.4.0  Replaced
    /// by: `tileRect`
    #[serde(rename = "__tileSrcRect")]
    pub(crate) tile_src_rect: Option<Vec<i64>>,

    /// Optional color
    pub(crate) color: i64,

    /// Enum value
    pub(crate) id: String,

    /// **WARNING**: this deprecated value is no longer exported since version 1.4.0  Replaced
    /// by: `tileRect`
    pub(crate) tile_id: Option<i64>,

    /// Optional tileset rectangle to represents this value
    pub(crate) tile_rect: Option<TilesetRectangle>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayerDefinition {
    /// Type of the layer (*IntGrid, Entities, Tiles or AutoLayer*)
    #[serde(rename = "__type")]
    pub(crate) layer_definition_type: String,

    /// Contains all the auto-layer rule definitions.
    pub(crate) auto_rule_groups: Vec<AutoLayerRuleGroup>,

    pub(crate) auto_source_layer_def_uid: Option<i64>,

    /// **WARNING**: this deprecated value is no longer exported since version 1.2.0  Replaced
    /// by: `tilesetDefUid`
    pub(crate) auto_tileset_def_uid: Option<i64>,

    /// Allow editor selections when the layer is not currently active.
    pub(crate) can_select_when_inactive: bool,

    /// Opacity of the layer (0 to 1.0)
    pub(crate) display_opacity: f64,

    /// User defined documentation for this element to provide help/tips to level designers.
    pub(crate) doc: Option<String>,

    /// An array of tags to forbid some Entities in this layer
    pub(crate) excluded_tags: Vec<String>,

    /// Width and height of the grid in pixels
    pub(crate) grid_size: i64,

    /// Height of the optional "guide" grid in pixels
    pub(crate) guide_grid_hei: i64,

    /// Width of the optional "guide" grid in pixels
    pub(crate) guide_grid_wid: i64,

    pub(crate) hide_fields_when_inactive: bool,

    /// Hide the layer from the list on the side of the editor view.
    pub(crate) hide_in_list: bool,

    /// User defined unique identifier
    pub(crate) identifier: String,

    /// Alpha of this layer when it is not the active one.
    pub(crate) inactive_opacity: f64,

    /// An array that defines extra optional info for each IntGrid value.<br/>  WARNING: the
    /// array order is not related to actual IntGrid values! As user can re-order IntGrid values
    /// freely, you may value "2" before value "1" in this array.
    pub(crate) int_grid_values: Vec<IntGridValueDefinition>,

    /// Group informations for IntGrid values
    pub(crate) int_grid_values_groups: Vec<IntGridValueGroupDefinition>,

    /// Parallax horizontal factor (from -1 to 1, defaults to 0) which affects the scrolling
    /// speed of this layer, creating a fake 3D (parallax) effect.
    pub(crate) parallax_factor_x: f64,

    /// Parallax vertical factor (from -1 to 1, defaults to 0) which affects the scrolling speed
    /// of this layer, creating a fake 3D (parallax) effect.
    pub(crate) parallax_factor_y: f64,

    /// If true (default), a layer with a parallax factor will also be scaled up/down accordingly.
    pub(crate) parallax_scaling: bool,

    /// X offset of the layer, in pixels (IMPORTANT: this should be added to the `LayerInstance`
    /// optional offset)
    pub(crate) px_offset_x: i64,

    /// Y offset of the layer, in pixels (IMPORTANT: this should be added to the `LayerInstance`
    /// optional offset)
    pub(crate) px_offset_y: i64,

    /// If TRUE, the content of this layer will be used when rendering levels in a simplified way
    /// for the world view
    pub(crate) render_in_world_view: bool,

    /// An array of tags to filter Entities that can be added to this layer
    pub(crate) required_tags: Vec<String>,

    /// If the tiles are smaller or larger than the layer grid, the pivot value will be used to
    /// position the tile relatively its grid cell.
    pub(crate) tile_pivot_x: f64,

    /// If the tiles are smaller or larger than the layer grid, the pivot value will be used to
    /// position the tile relatively its grid cell.
    pub(crate) tile_pivot_y: f64,

    /// Reference to the default Tileset UID being used by this layer definition.<br/>
    /// **WARNING**: some layer *instances* might use a different tileset. So most of the time,
    /// you should probably use the `__tilesetDefUid` value found in layer instances.<br/>  Note:
    /// since version 1.0.0, the old `autoTilesetDefUid` was removed and merged into this value.
    pub(crate) tileset_def_uid: Option<i64>,

    /// Type of the layer as Haxe Enum Possible values: `IntGrid`, `Entities`, `Tiles`,
    /// `AutoLayer`
    #[serde(rename = "type")]
    pub(crate) purple_type: Type,

    /// User defined color for the UI
    pub(crate) ui_color: Option<String>,

    /// Unique Int identifier
    pub(crate) uid: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoLayerRuleGroup {
    pub(crate) active: bool,

    /// *This field was removed in 1.0.0 and should no longer be used.*
    pub(crate) collapsed: Option<bool>,

    pub(crate) color: Option<String>,

    pub(crate) icon: Option<TilesetRectangle>,

    pub(crate) is_optional: bool,

    pub(crate) name: String,

    pub(crate) rules: Vec<AutoLayerRuleDefinition>,

    pub(crate) uid: i64,

    pub(crate) uses_wizard: bool,
}

/// This complex section isn't meant to be used by game devs at all, as these rules are
/// completely resolved internally by the editor before any saving. You should just ignore
/// this part.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoLayerRuleDefinition {
    /// If FALSE, the rule effect isn't applied, and no tiles are generated.
    pub(crate) active: bool,

    pub(crate) alpha: f64,

    /// When TRUE, the rule will prevent other rules to be applied in the same cell if it matches
    /// (TRUE by default).
    pub(crate) break_on_match: bool,

    /// Chances for this rule to be applied (0 to 1)
    pub(crate) chance: f64,

    /// Checker mode Possible values: `None`, `Horizontal`, `Vertical`
    pub(crate) checker: Checker,

    /// If TRUE, allow rule to be matched by flipping its pattern horizontally
    pub(crate) flip_x: bool,

    /// If TRUE, allow rule to be matched by flipping its pattern vertically
    pub(crate) flip_y: bool,

    /// Default IntGrid value when checking cells outside of level bounds
    pub(crate) out_of_bounds_value: Option<i64>,

    /// Rule pattern (size x size)
    pub(crate) pattern: Vec<i64>,

    /// If TRUE, enable Perlin filtering to only apply rule on specific random area
    pub(crate) perlin_active: bool,

    pub(crate) perlin_octaves: f64,

    pub(crate) perlin_scale: f64,

    pub(crate) perlin_seed: f64,

    /// X pivot of a tile stamp (0-1)
    pub(crate) pivot_x: f64,

    /// Y pivot of a tile stamp (0-1)
    pub(crate) pivot_y: f64,

    /// Pattern width & height. Should only be 1,3,5 or 7.
    pub(crate) size: i64,

    /// Array of all the tile IDs. They are used randomly or as stamps, based on `tileMode` value.
    pub(crate) tile_ids: Vec<i64>,

    /// Defines how tileIds array is used Possible values: `Single`, `Stamp`
    pub(crate) tile_mode: TileMode,

    /// Max random offset for X tile pos
    pub(crate) tile_random_x_max: i64,

    /// Min random offset for X tile pos
    pub(crate) tile_random_x_min: i64,

    /// Max random offset for Y tile pos
    pub(crate) tile_random_y_max: i64,

    /// Min random offset for Y tile pos
    pub(crate) tile_random_y_min: i64,

    /// Tile X offset
    pub(crate) tile_x_offset: i64,

    /// Tile Y offset
    pub(crate) tile_y_offset: i64,

    /// Unique Int identifier
    pub(crate) uid: i64,

    /// X cell coord modulo
    pub(crate) x_modulo: i64,

    /// X cell start offset
    pub(crate) x_offset: i64,

    /// Y cell coord modulo
    pub(crate) y_modulo: i64,

    /// Y cell start offset
    pub(crate) y_offset: i64,
}

/// Checker mode Possible values: `None`, `Horizontal`, `Vertical`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Checker {
    Horizontal,

    None,

    Vertical,
}

/// Defines how tileIds array is used Possible values: `Single`, `Stamp`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TileMode {
    Single,

    Stamp,
}

/// IntGrid value definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntGridValueDefinition {
    pub(crate) color: String,

    /// Parent group identifier (0 if none)
    pub(crate) group_uid: i64,

    /// User defined unique identifier
    pub(crate) identifier: Option<String>,

    pub(crate) tile: Option<TilesetRectangle>,

    /// The IntGrid value itself
    pub(crate) value: i64,
}

/// IntGrid value group definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntGridValueGroupDefinition {
    /// User defined color
    pub(crate) color: Option<String>,

    /// User defined string identifier
    pub(crate) identifier: Option<String>,

    /// Group unique ID
    pub(crate) uid: i64,
}

/// Type of the layer as Haxe Enum Possible values: `IntGrid`, `Entities`, `Tiles`,
/// `AutoLayer`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "AutoLayer")]
    AutoLayer,

    Entities,

    #[serde(rename = "IntGrid")]
    IntGrid,

    Tiles,
}

/// The `Tileset` definition is the most important part among project definitions. It
/// contains some extra informations about each integrated tileset. If you only had to parse
/// one definition section, that would be the one.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TilesetDefinition {
    /// Grid-based height
    #[serde(rename = "__cHei")]
    pub(crate) c_hei: i64,

    /// Grid-based width
    #[serde(rename = "__cWid")]
    pub(crate) c_wid: i64,

    /// The following data is used internally for various optimizations. It's always synced with
    /// source image changes.
    pub(crate) cached_pixel_data: Option<HashMap<String, Option<serde_json::Value>>>,

    /// An array of custom tile metadata
    pub(crate) custom_data: Vec<TileCustomMetadata>,

    /// If this value is set, then it means that this atlas uses an internal LDtk atlas image
    /// instead of a loaded one. Possible values: &lt;`null`&gt;, `LdtkIcons`
    pub(crate) embed_atlas: Option<EmbedAtlas>,

    /// Tileset tags using Enum values specified by `tagsSourceEnumId`. This array contains 1
    /// element per Enum value, which contains an array of all Tile IDs that are tagged with it.
    pub(crate) enum_tags: Vec<EnumTagValue>,

    /// User defined unique identifier
    pub(crate) identifier: String,

    /// Distance in pixels from image borders
    pub(crate) padding: i64,

    /// Image height in pixels
    pub(crate) px_hei: i64,

    /// Image width in pixels
    pub(crate) px_wid: i64,

    /// Path to the source file, relative to the current project JSON file<br/>  It can be null
    /// if no image was provided, or when using an embed atlas.
    pub(crate) rel_path: Option<String>,

    /// Array of group of tiles selections, only meant to be used in the editor
    pub(crate) saved_selections: Vec<HashMap<String, Option<serde_json::Value>>>,

    /// Space in pixels between all tiles
    pub(crate) spacing: i64,

    /// An array of user-defined tags to organize the Tilesets
    pub(crate) tags: Vec<String>,

    /// Optional Enum definition UID used for this tileset meta-data
    pub(crate) tags_source_enum_uid: Option<i64>,

    pub(crate) tile_grid_size: i64,

    /// Unique Intidentifier
    pub(crate) uid: i64,
}

/// In a tileset definition, user defined meta-data of a tile.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TileCustomMetadata {
    pub(crate) data: String,

    pub(crate) tile_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EmbedAtlas {
    #[serde(rename = "LdtkIcons")]
    LdtkIcons,
}

/// In a tileset definition, enum based tag infos
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnumTagValue {
    pub(crate) enum_value_id: String,

    pub(crate) tile_ids: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Flag {
    #[serde(rename = "DiscardPreCsvIntGrid")]
    DiscardPreCsvIntGrid,

    #[serde(rename = "ExportPreCsvIntGridFormat")]
    ExportPreCsvIntGridFormat,

    #[serde(rename = "IgnoreBackupSuggest")]
    IgnoreBackupSuggest,

    #[serde(rename = "MultiWorlds")]
    MultiWorlds,

    #[serde(rename = "PrependIndexToLevelFileNames")]
    PrependIndexToLevelFileNames,

    #[serde(rename = "UseMultilinesType")]
    UseMultilinesType,
}

/// This object is not actually used by LDtk. It ONLY exists to force explicit references to
/// all types, to make sure QuickType finds them and integrate all of them. Otherwise,
/// Quicktype will drop types that are not explicitely used.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ForcedRefs {
    pub(crate) auto_layer_rule_group: Option<AutoLayerRuleGroup>,

    pub(crate) auto_rule_def: Option<AutoLayerRuleDefinition>,

    pub(crate) custom_command: Option<LdtkCustomCommand>,

    pub(crate) definitions: Option<Definitions>,

    pub(crate) entity_def: Option<EntityDefinition>,

    pub(crate) entity_instance: Option<EntityInstance>,

    pub(crate) entity_reference_infos: Option<ReferenceToAnEntityInstance>,

    pub(crate) enum_def: Option<EnumDefinition>,

    pub(crate) enum_def_values: Option<EnumValueDefinition>,

    pub(crate) enum_tag_value: Option<EnumTagValue>,

    pub(crate) field_def: Option<FieldDefinition>,

    pub(crate) field_instance: Option<FieldInstance>,

    pub(crate) grid_point: Option<GridPoint>,

    pub(crate) int_grid_value_def: Option<IntGridValueDefinition>,

    pub(crate) int_grid_value_group_def: Option<IntGridValueGroupDefinition>,

    pub(crate) int_grid_value_instance: Option<IntGridValueInstance>,

    pub(crate) layer_def: Option<LayerDefinition>,

    pub(crate) layer_instance: Option<LayerInstance>,

    pub(crate) level: Option<Level>,

    pub(crate) level_bg_pos_infos: Option<LevelBackgroundPosition>,

    pub(crate) neighbour_level: Option<NeighbourLevel>,

    pub(crate) table_of_content_entry: Option<LdtkTableOfContentEntry>,

    pub(crate) tile: Option<TileInstance>,

    pub(crate) tile_custom_metadata: Option<TileCustomMetadata>,

    pub(crate) tileset_def: Option<TilesetDefinition>,

    pub(crate) tileset_rect: Option<TilesetRectangle>,

    pub(crate) world: Option<World>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityInstance {
    /// Grid-based coordinates (`[x,y]` format)
    #[serde(rename = "__grid")]
    pub(crate) grid: Vec<i64>,

    /// Entity definition identifier
    #[serde(rename = "__identifier")]
    pub(crate) identifier: String,

    /// Pivot coordinates  (`[x,y]` format, values are from 0 to 1) of the Entity
    #[serde(rename = "__pivot")]
    pub(crate) pivot: Vec<f64>,

    /// The entity "smart" color, guessed from either Entity definition, or one its field
    /// instances.
    #[serde(rename = "__smartColor")]
    pub(crate) smart_color: String,

    /// Array of tags defined in this Entity definition
    #[serde(rename = "__tags")]
    pub(crate) tags: Vec<String>,

    /// Optional TilesetRect used to display this entity (it could either be the default Entity
    /// tile, or some tile provided by a field value, like an Enum).
    #[serde(rename = "__tile")]
    pub(crate) tile: Option<TilesetRectangle>,

    /// X world coordinate in pixels
    #[serde(rename = "__worldX")]
    pub(crate) world_x: i64,

    /// Y world coordinate in pixels
    #[serde(rename = "__worldY")]
    pub(crate) world_y: i64,

    /// Reference of the **Entity definition** UID
    pub(crate) def_uid: i64,

    /// An array of all custom fields and their values.
    pub(crate) field_instances: Vec<FieldInstance>,

    /// Entity height in pixels. For non-resizable entities, it will be the same as Entity
    /// definition.
    pub(crate) height: i64,

    /// Unique instance identifier
    pub(crate) iid: String,

    /// Pixel coordinates (`[x,y]` format) in current level coordinate space. Don't forget
    /// optional layer offsets, if they exist!
    pub(crate) px: Vec<i64>,

    /// Entity width in pixels. For non-resizable entities, it will be the same as Entity
    /// definition.
    pub(crate) width: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldInstance {
    /// Field definition identifier
    #[serde(rename = "__identifier")]
    pub(crate) identifier: String,

    /// Optional TilesetRect used to display this field (this can be the field own Tile, or some
    /// other Tile guessed from the value, like an Enum).
    #[serde(rename = "__tile")]
    pub(crate) tile: Option<TilesetRectangle>,

    /// Type of the field, such as `Int`, `Float`, `String`, `Enum(my_enum_name)`, `Bool`,
    /// etc.<br/>  NOTE: if you enable the advanced option **Use Multilines type**, you will have
    /// "*Multilines*" instead of "*String*" when relevant.
    #[serde(rename = "__type")]
    pub(crate) field_instance_type: String,

    /// Actual value of the field instance. The value type varies, depending on `__type`:<br/>
    /// - For **classic types** (ie. Integer, Float, Boolean, String, Text and FilePath), you
    /// just get the actual value with the expected type.<br/>   - For **Color**, the value is an
    /// hexadecimal string using "#rrggbb" format.<br/>   - For **Enum**, the value is a String
    /// representing the selected enum value.<br/>   - For **Point**, the value is a
    /// [GridPoint](#ldtk-GridPoint) object.<br/>   - For **Tile**, the value is a
    /// [TilesetRect](#ldtk-TilesetRect) object.<br/>   - For **EntityRef**, the value is an
    /// [EntityReferenceInfos](#ldtk-EntityReferenceInfos) object.<br/><br/>  If the field is an
    /// array, then this `__value` will also be a JSON array.
    #[serde(rename = "__value")]
    pub(crate) value: Option<serde_json::Value>,

    /// Reference of the **Field definition** UID
    pub(crate) def_uid: i64,

    /// Editor internal raw values
    pub(crate) real_editor_values: Vec<Option<serde_json::Value>>,
}

/// This object describes the "location" of an Entity instance in the project worlds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceToAnEntityInstance {
    /// IID of the refered EntityInstance
    pub(crate) entity_iid: String,

    /// IID of the LayerInstance containing the refered EntityInstance
    pub(crate) layer_iid: String,

    /// IID of the Level containing the refered EntityInstance
    pub(crate) level_iid: String,

    /// IID of the World containing the refered EntityInstance
    pub(crate) world_iid: String,
}

/// This object is just a grid-based coordinate used in Field values.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GridPoint {
    /// X grid-based coordinate
    pub(crate) cx: i64,

    /// Y grid-based coordinate
    pub(crate) cy: i64,
}

/// IntGrid value instance
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntGridValueInstance {
    /// Coordinate ID in the layer grid
    pub(crate) coord_id: i64,

    /// IntGrid value
    pub(crate) v: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayerInstance {
    /// Grid-based height
    #[serde(rename = "__cHei")]
    pub(crate) c_hei: i64,

    /// Grid-based width
    #[serde(rename = "__cWid")]
    pub(crate) c_wid: i64,

    /// Grid size
    #[serde(rename = "__gridSize")]
    pub(crate) grid_size: i64,

    /// Layer definition identifier
    #[serde(rename = "__identifier")]
    pub(crate) identifier: String,

    /// Layer opacity as Float [0-1]
    #[serde(rename = "__opacity")]
    pub(crate) opacity: f64,

    /// Total layer X pixel offset, including both instance and definition offsets.
    #[serde(rename = "__pxTotalOffsetX")]
    pub(crate) px_total_offset_x: i64,

    /// Total layer Y pixel offset, including both instance and definition offsets.
    #[serde(rename = "__pxTotalOffsetY")]
    pub(crate) px_total_offset_y: i64,

    /// The definition UID of corresponding Tileset, if any.
    #[serde(rename = "__tilesetDefUid")]
    pub(crate) tileset_def_uid: Option<i64>,

    /// The relative path to corresponding Tileset, if any.
    #[serde(rename = "__tilesetRelPath")]
    pub(crate) tileset_rel_path: Option<String>,

    /// Layer type (possible values: IntGrid, Entities, Tiles or AutoLayer)
    #[serde(rename = "__type")]
    pub(crate) layer_instance_type: String,

    /// An array containing all tiles generated by Auto-layer rules. The array is already sorted
    /// in display order (ie. 1st tile is beneath 2nd, which is beneath 3rd etc.).<br/><br/>
    /// Note: if multiple tiles are stacked in the same cell as the result of different rules,
    /// all tiles behind opaque ones will be discarded.
    pub(crate) auto_layer_tiles: Vec<TileInstance>,

    pub(crate) entity_instances: Vec<EntityInstance>,

    pub(crate) grid_tiles: Vec<TileInstance>,

    /// Unique layer instance identifier
    pub(crate) iid: String,

    /// **WARNING**: this deprecated value is no longer exported since version 1.0.0  Replaced
    /// by: `intGridCsv`
    pub(crate) int_grid: Option<Vec<IntGridValueInstance>>,

    /// A list of all values in the IntGrid layer, stored in CSV format (Comma Separated
    /// Values).<br/>  Order is from left to right, and top to bottom (ie. first row from left to
    /// right, followed by second row, etc).<br/>  `0` means "empty cell" and IntGrid values
    /// start at 1.<br/>  The array size is `__cWid` x `__cHei` cells.
    pub(crate) int_grid_csv: Vec<i64>,

    /// Reference the Layer definition UID
    pub(crate) layer_def_uid: i64,

    /// Reference to the UID of the level containing this layer instance
    pub(crate) level_id: i64,

    /// An Array containing the UIDs of optional rules that were enabled in this specific layer
    /// instance.
    pub(crate) optional_rules: Vec<i64>,

    /// This layer can use another tileset by overriding the tileset UID here.
    pub(crate) override_tileset_uid: Option<i64>,

    /// X offset in pixels to render this layer, usually 0 (IMPORTANT: this should be added to
    /// the `LayerDef` optional offset, so you should probably prefer using `__pxTotalOffsetX`
    /// which contains the total offset value)
    pub(crate) px_offset_x: i64,

    /// Y offset in pixels to render this layer, usually 0 (IMPORTANT: this should be added to
    /// the `LayerDef` optional offset, so you should probably prefer using `__pxTotalOffsetX`
    /// which contains the total offset value)
    pub(crate) px_offset_y: i64,

    /// Random seed used for Auto-Layers rendering
    pub(crate) seed: i64,

    /// Layer instance visibility
    pub(crate) visible: bool,
}

/// This structure represents a single tile from a given Tileset.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TileInstance {
    /// Alpha/opacity of the tile (0-1, defaults to 1)
    pub(crate) a: f64,

    /// Internal data used by the editor.<br/>  For auto-layer tiles: `[ruleId, coordId]`.<br/>
    /// For tile-layer tiles: `[coordId]`.
    pub(crate) d: Vec<i64>,

    /// "Flip bits", a 2-bits integer to represent the mirror transformations of the tile.<br/>
    /// - Bit 0 = X flip<br/>   - Bit 1 = Y flip<br/>   Examples: f=0 (no flip), f=1 (X flip
    /// only), f=2 (Y flip only), f=3 (both flips)
    pub(crate) f: i64,

    /// Pixel coordinates of the tile in the **layer** (`[x,y]` format). Don't forget optional
    /// layer offsets, if they exist!
    pub(crate) px: Vec<i64>,

    /// Pixel coordinates of the tile in the **tileset** (`[x,y]` format)
    pub(crate) src: Vec<i64>,

    /// The *Tile ID* in the corresponding tileset.
    pub(crate) t: i64,
}

/// This section contains all the level data. It can be found in 2 distinct forms, depending
/// on Project current settings:  - If "*Separate level files*" is **disabled** (default):
/// full level data is *embedded* inside the main Project JSON file, - If "*Separate level
/// files*" is **enabled**: level data is stored in *separate* standalone `.ldtkl` files (one
/// per level). In this case, the main Project JSON file will still contain most level data,
/// except heavy sections, like the `layerInstances` array (which will be null). The
/// `externalRelPath` string points to the `ldtkl` file.  A `ldtkl` file is just a JSON file
/// containing exactly what is described below.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Level {
    /// Background color of the level (same as `bgColor`, except the default value is
    /// automatically used here if its value is `null`)
    #[serde(rename = "__bgColor")]
    pub(crate) bg_color: String,

    /// Position informations of the background image, if there is one.
    #[serde(rename = "__bgPos")]
    pub(crate) bg_pos: Option<LevelBackgroundPosition>,

    /// An array listing all other levels touching this one on the world map. Since 1.4.0, this
    /// includes levels that overlap in the same world layer, or in nearby world layers.<br/>
    /// Only relevant for world layouts where level spatial positioning is manual (ie. GridVania,
    /// Free). For Horizontal and Vertical layouts, this array is always empty.
    #[serde(rename = "__neighbours")]
    pub(crate) neighbours: Vec<NeighbourLevel>,

    /// The "guessed" color for this level in the editor, decided using either the background
    /// color or an existing custom field.
    #[serde(rename = "__smartColor")]
    pub(crate) smart_color: String,

    /// Background color of the level. If `null`, the project `defaultLevelBgColor` should be
    /// used.
    #[serde(rename = "bgColor")]
    pub(crate) level_bg_color: Option<String>,

    /// Background image X pivot (0-1)
    pub(crate) bg_pivot_x: f64,

    /// Background image Y pivot (0-1)
    pub(crate) bg_pivot_y: f64,

    /// An enum defining the way the background image (if any) is positioned on the level. See
    /// `__bgPos` for resulting position info. Possible values: &lt;`null`&gt;, `Unscaled`,
    /// `Contain`, `Cover`, `CoverDirty`, `Repeat`
    #[serde(rename = "bgPos")]
    pub(crate) level_bg_pos: Option<BgPos>,

    /// The *optional* relative path to the level background image.
    pub(crate) bg_rel_path: Option<String>,

    /// This value is not null if the project option "*Save levels separately*" is enabled. In
    /// this case, this **relative** path points to the level Json file.
    pub(crate) external_rel_path: Option<String>,

    /// An array containing this level custom field values.
    pub(crate) field_instances: Vec<FieldInstance>,

    /// User defined unique identifier
    pub(crate) identifier: String,

    /// Unique instance identifier
    pub(crate) iid: String,

    /// An array containing all Layer instances. **IMPORTANT**: if the project option "*Save
    /// levels separately*" is enabled, this field will be `null`.<br/>  This array is **sorted
    /// in display order**: the 1st layer is the top-most and the last is behind.
    pub(crate) layer_instances: Option<Vec<LayerInstance>>,

    /// Height of the level in pixels
    pub(crate) px_hei: i64,

    /// Width of the level in pixels
    pub(crate) px_wid: i64,

    /// Unique Int identifier
    pub(crate) uid: i64,

    /// If TRUE, the level identifier will always automatically use the naming pattern as defined
    /// in `Project.levelNamePattern`. Becomes FALSE if the identifier is manually modified by
    /// user.
    pub(crate) use_auto_identifier: bool,

    /// Index that represents the "depth" of the level in the world. Default is 0, greater means
    /// "above", lower means "below".<br/>  This value is mostly used for display only and is
    /// intended to make stacking of levels easier to manage.
    pub(crate) world_depth: i64,

    /// World X coordinate in pixels.<br/>  Only relevant for world layouts where level spatial
    /// positioning is manual (ie. GridVania, Free). For Horizontal and Vertical layouts, the
    /// value is always -1 here.
    pub(crate) world_x: i64,

    /// World Y coordinate in pixels.<br/>  Only relevant for world layouts where level spatial
    /// positioning is manual (ie. GridVania, Free). For Horizontal and Vertical layouts, the
    /// value is always -1 here.
    pub(crate) world_y: i64,
}

/// Level background image position info
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LevelBackgroundPosition {
    /// An array of 4 float values describing the cropped sub-rectangle of the displayed
    /// background image. This cropping happens when original is larger than the level bounds.
    /// Array format: `[ cropX, cropY, cropWidth, cropHeight ]`
    pub(crate) crop_rect: Vec<f64>,

    /// An array containing the `[scaleX,scaleY]` values of the **cropped** background image,
    /// depending on `bgPos` option.
    pub(crate) scale: Vec<f64>,

    /// An array containing the `[x,y]` pixel coordinates of the top-left corner of the
    /// **cropped** background image, depending on `bgPos` option.
    pub(crate) top_left_px: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BgPos {
    Contain,

    Cover,

    #[serde(rename = "CoverDirty")]
    CoverDirty,

    Repeat,

    Unscaled,
}

/// Nearby level info
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NeighbourLevel {
    /// A single lowercase character tipping on the level location (`n`orth, `s`outh, `w`est,
    /// `e`ast).<br/>  Since 1.4.0, this character value can also be `<` (neighbour depth is
    /// lower), `>` (neighbour depth is greater) or `o` (levels overlap and share the same world
    /// depth).
    pub(crate) dir: String,

    /// Neighbour Instance Identifier
    pub(crate) level_iid: String,

    /// **WARNING**: this deprecated value is no longer exported since version 1.2.0  Replaced
    /// by: `levelIid`
    pub(crate) level_uid: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LdtkTableOfContentEntry {
    pub(crate) identifier: String,

    pub(crate) instances: Vec<ReferenceToAnEntityInstance>,
}

/// **IMPORTANT**: this type is available as a preview. You can rely on it to update your
/// importers, for when it will be officially available.  A World contains multiple levels,
/// and it has its own layout settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct World {
    /// Default new level height
    pub(crate) default_level_height: i64,

    /// Default new level width
    pub(crate) default_level_width: i64,

    /// User defined unique identifier
    pub(crate) identifier: String,

    /// Unique instance identifer
    pub(crate) iid: String,

    /// All levels from this world. The order of this array is only relevant in
    /// `LinearHorizontal` and `linearVertical` world layouts (see `worldLayout` value).
    /// Otherwise, you should refer to the `worldX`,`worldY` coordinates of each Level.
    pub(crate) levels: Vec<Level>,

    /// Height of the world grid in pixels.
    pub(crate) world_grid_height: i64,

    /// Width of the world grid in pixels.
    pub(crate) world_grid_width: i64,

    /// An enum that describes how levels are organized in this project (ie. linearly or in a 2D
    /// space). Possible values: `Free`, `GridVania`, `LinearHorizontal`, `LinearVertical`, `null`
    pub(crate) world_layout: Option<WorldLayout>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WorldLayout {
    Free,

    #[serde(rename = "GridVania")]
    GridVania,

    #[serde(rename = "LinearHorizontal")]
    LinearHorizontal,

    #[serde(rename = "LinearVertical")]
    LinearVertical,
}

/// Naming convention for Identifiers (first-letter uppercase, full uppercase etc.) Possible
/// values: `Capitalize`, `Uppercase`, `Lowercase`, `Free`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IdentifierStyle {
    Capitalize,

    Free,

    Lowercase,

    Uppercase,
}

/// "Image export" option when saving project. Possible values: `None`, `OneImagePerLayer`,
/// `OneImagePerLevel`, `LayersAndLevels`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ImageExportMode {
    #[serde(rename = "LayersAndLevels")]
    LayersAndLevels,

    None,

    #[serde(rename = "OneImagePerLayer")]
    OneImagePerLayer,

    #[serde(rename = "OneImagePerLevel")]
    OneImagePerLevel,
}

