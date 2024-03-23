use bevy::prelude::*;

use crate::{ldtk::IntGridValueDefinition, prelude::Layer};

/// A system parameter for querying int grid values  
/// at a given location in world space.
pub type IntGridAtLocation<'world, 'state> =
    Query<'world, 'state, (&'static GlobalTransform, &'static Layer)>;

#[allow(missing_docs)]
pub trait IntGridAtLocationTrait {
    fn all(&self, location: Vec2) -> Vec<IntGridValueDefinition>;
    fn top(&self, location: Vec2) -> Option<IntGridValueDefinition>;
}

impl IntGridAtLocationTrait for IntGridAtLocation<'_, '_> {
    fn all(&self, location: Vec2) -> Vec<IntGridValueDefinition> {
        // build and allocate a list of all layers which contain the location
        // We need to allocate so we can .sort_by(...) later
        let mut filtered_layers: Vec<_> = self
            .iter()
            .filter(|(transform, layer)| {
                let layer_position = transform.translation().truncate();
                let layer_size = Vec2::new(1.0, -1.0) * layer.size();
                Rect::from_corners(layer_position, layer_position + layer_size).contains(location)
            })
            .collect();
        // sort from top to bottom by z layer
        filtered_layers.sort_by(|(a, _), (b, _)| {
            b.translation()
                .z
                .partial_cmp(&a.translation().z)
                .expect("Unable to sort layers?")
        });
        // now filter for layers which actually have an int grid at given location
        filtered_layers
            .iter()
            .filter_map(|(transform, layer)| {
                // debug!("layer: {layer:?}");
                let location_local = location - transform.translation().truncate();
                let row = (location_local.x.floor() as i64) / layer.grid_width();
                let col = (-location_local.y.floor() as i64) / layer.grid_height();
                // debug!("row, col: ({row}, {col})");
                let index = (row + col * layer.grid_width()) as usize;
                let value = layer.int_grid_csv().get(index);
                match value {
                    Some(Some(n)) => Some(n),
                    Some(None) | None => None,
                }
            })
            .cloned()
            .collect()
    }

    fn top(&self, location: Vec2) -> Option<IntGridValueDefinition> {
        // build and allocate a list of all layers which contain the location
        // We need to allocate so we can .sort_by(...) later
        let mut filtered_layers: Vec<_> = self
            .iter()
            .filter(|(transform, layer)| {
                let layer_position = transform.translation().truncate();
                let layer_size = Vec2::new(1.0, -1.0) * layer.size();
                Rect::from_corners(layer_position, layer_position + layer_size).contains(location)
            })
            .collect();
        // sort from top to bottom by z layer
        filtered_layers.sort_by(|(a, _), (b, _)| {
            b.translation()
                .z
                .partial_cmp(&a.translation().z)
                .expect("Unable to sort layers?")
        });
        filtered_layers
            .iter()
            .find_map(|(transform, layer)| {
                // debug!("layer: {layer:?}");
                let location_local = location - transform.translation().truncate();
                let row = (location_local.x.floor() as i64) / layer.grid_width();
                let col = (-location_local.y.floor() as i64) / layer.grid_height();
                // debug!("row, col: ({row}, {col})");
                let index = (row + col * layer.grid_width()) as usize;
                let value = layer.int_grid_csv().get(index);
                match value {
                    Some(Some(n)) => Some(n),
                    Some(None) | None => None,
                }
            })
            .cloned()
    }
}
