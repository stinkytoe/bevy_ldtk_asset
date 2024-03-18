use bevy::prelude::*;

use crate::{
    ldtk,
    prelude::{HasIdentifier, ProjectAsset},
    util::bevy_color_from_ldtk,
};

/// A component included in WorldBundle, which we will use to determine if a given
/// asset should spawn its associated entities, or simply be loaded as data
#[derive(Component, Default)]
pub enum SpawnEntities {
    #[default]
    /// Load nothing
    Nothing,
    /// Load all entities
    Everything,
}

#[derive(Component, Default)]
pub struct BackgroundColor;

#[derive(Component, Default)]
pub struct BackgroundImage;

/// A component attached to all level layers
#[derive(Component, Default)]
pub struct Layer {
    grid_width: i64,
    grid_height: i64,
    grid_size: i64,
    identifier: String,
    iid: String,
    int_grid_csv: Vec<Option<ldtk::IntGridValueDefinition>>,
    layer_def_uid: i64,
    level_id: i64,
}

impl HasIdentifier for Layer {
    fn identifier(&self) -> &str {
        &self.identifier
    }
}

impl Layer {
    pub(crate) fn new(
        layer: &ldtk::LayerInstance,
        layer_definition: &ldtk::LayerDefinition,
    ) -> Self {
        Self {
            grid_width: layer.c_wid,
            grid_height: layer.c_hei,
            grid_size: layer.grid_size,
            identifier: layer.identifier.clone(),
            iid: layer.iid.clone(),
            int_grid_csv: layer
                .int_grid_csv
                .iter()
                .map(|i| {
                    layer_definition
                        .int_grid_values
                        .iter()
                        .find(|int_grid_value| int_grid_value.value == *i)
                        .cloned()
                })
                .collect(),
            layer_def_uid: layer.layer_def_uid,
            level_id: layer.level_id,
        }
    }

    /// Returns the width of the cells in this layer
    pub fn grid_width(&self) -> i64 {
        self.grid_width
    }

    /// Returns the height of the cells in this layer
    pub fn grid_height(&self) -> i64 {
        self.grid_height
    }

    /// Returns the size in pixels of a grid square in this layer
    pub fn grid_size(&self) -> i64 {
        self.grid_size
    }

    /// Returns the integer identifier of this layer
    pub fn iid(&self) -> &str {
        self.iid.as_ref()
    }

    /// The array containing the int grids in this layer, if any
    pub fn int_grid_csv(&self) -> &[Option<ldtk::IntGridValueDefinition>] {
        self.int_grid_csv.as_ref()
    }

    /// The uid of the layer definition
    pub fn layer_def_uid(&self) -> i64 {
        self.layer_def_uid
    }

    /// The id of the level which contains this layer instance
    pub fn level_id(&self) -> i64 {
        self.level_id
    }
}

/// A layer containing ldtk entities
#[derive(Component, Default)]
pub struct LdtkEntityLayer {}

/// An LDtk entity as a Bevy component.
#[derive(Component, Default)]
pub struct LdtkEntity {
    project_handle: Handle<ProjectAsset>,
    identifier: String,
    pivot: Vec2,
    smart_color: Color,
    tags: Vec<String>,
    field_instances: Vec<ldtk::FieldInstance>,
    size: Vec2,
    iid: String,
}

impl LdtkEntity {
    pub(crate) fn new(value: &ldtk::EntityInstance, project_handle: Handle<ProjectAsset>) -> Self {
        Self {
            project_handle,
            identifier: value.identifier.clone(),
            pivot: Vec2::new(value.pivot[0] as f32, value.pivot[1] as f32),
            smart_color: bevy_color_from_ldtk(&value.smart_color).unwrap_or_default(),
            tags: value.tags.clone(),
            field_instances: value.field_instances.clone(),
            size: Vec2::new(value.width as f32, value.height as f32),
            iid: value.iid.clone(),
        }
    }

    /// Returns the identifier for this entity, as defined in the LDtk project
    pub fn identifier(&self) -> &str {
        self.identifier.as_ref()
    }

    /// The pivot, i.e. the offset of the entities visual representation from its
    /// 'center.' Represented as a number from 0 to 1, commonly from a set of
    /// [0.0, 0.5, 1.0], with 0.5 on both axes meaning the visual should be centered
    /// around its location.
    pub fn pivot(&self) -> Vec2 {
        self.pivot
    }

    /// The 'smart color,' or the color which could be used to represent a zoomed
    /// out version of the sprite for this image.
    pub fn smart_color(&self) -> Color {
        self.smart_color
    }

    /// Returns a vector of all tags associated with this LDtk entity instance.
    pub fn tags(&self) -> &[String] {
        self.tags.as_ref()
    }

    /// Returns a vector of the field instances associated with this LDtk entity instance.
    pub fn field_instances(&self) -> &[ldtk::FieldInstance] {
        self.field_instances.as_ref()
    }

    /// The size of this entity. Usually the size of a cell in the level it was created in,
    /// but can be user defined per eitity instance.
    pub fn size(&self) -> Vec2 {
        self.size
    }

    /// The unique identifier for this entity instance, generated by LDtk.
    pub fn iid(&self) -> &str {
        self.iid.as_ref()
    }

    /// A handle to the ProjectAsset which this ldtk entity was defined in.
    pub fn project_handle(&self) -> &Handle<ProjectAsset> {
        &self.project_handle
    }
}

impl LdtkEntity {
    /// Checks a given ldtk entity component for the presense of the given tag.
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|inner_tag| inner_tag == tag)
    }
}
