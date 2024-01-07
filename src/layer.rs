use bevy::prelude::*;

use crate::ldtk_json;

/// A read-only object which represents the layer definition
/// as defined in the LDtk project.
pub struct LdtkLayerDefinition<'a> {
    ///
    pub value: &'a ldtk_json::LayerDefinition,
}

/// A read-only object which represents the layer instance
/// as defined in the LDtk project.
pub struct LdtkLayerInstance<'a> {
    ///
    pub value: &'a ldtk_json::LayerInstance,
}

impl LdtkLayerInstance<'_> {
    /// Returns the grid holding the values of the int_grid, if any,
    /// which was defined in the LDtk project.
    pub fn int_grid_csv(&self) -> &Vec<i64> {
        &self.value.int_grid_csv
    }

    /// The size of the logical grid for this layer.
    pub fn grid_size(&self) -> i64 {
        self.value.grid_size
    }

    /// The width of the tiles/cells in this layer.
    pub fn c_wid(&self) -> i64 {
        self.value.c_wid
    }

    /// The height of the tiles/cells in this layer.
    pub fn c_hei(&self) -> i64 {
        self.value.c_hei
    }

    /// Returns the row and column represented by the given index.
    /// Returns Some((row, col)) if the index is in bounds, or None if
    /// out of bounds.
    pub fn get_grid_coordinate_from_index(&self, index: usize) -> Option<(i64, i64)> {
        let row = index as i64 % self.c_wid();
        let col = index as i64 / self.c_wid();
        if (0..self.grid_size()).contains(&row) || (0..self.grid_size()).contains(&col) {
            Some((row, col))
        } else {
            None
        }
    }

    /// Returns the top-left coordinate of a given index.
    /// Returns Some(coordinate) if the index is in bounds, or None if
    /// out of bounds.
    pub fn get_level_coordinate_from_index(&self, index: usize) -> Option<Vec3> {
        let (r, c) = self.get_grid_coordinate_from_index(index)?;

        let x = self.grid_size() * r;
        let y = self.grid_size() * c;

        Some(Vec3::new(x as f32, -y as f32, 0.0))
    }
}
