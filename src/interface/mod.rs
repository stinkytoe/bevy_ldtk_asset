use bevy::prelude::*;

use crate::{ldtk, prelude::*};

#[allow(missing_docs)]
pub type LevelAtLocationQuery<'a, 'b> =
    Query<'a, 'b, (&'static GlobalTransform, &'static Handle<LevelAsset>)>;

/// Finds all levels whose surface covers the given position in global space
pub fn levels_at_location(
    location: Vec2,
    levels: &Assets<LevelAsset>,
    levels_query: LevelAtLocationQuery,
) -> Vec<Handle<LevelAsset>> {
    levels_query
        .iter()
        .filter(|(transform, handle)| {
            let level = levels.get(*handle).expect("bad level handle!");
            let level_position = transform.translation().truncate();
            let level_size = Vec2::new(1.0, -1.0) * level.size();
            Rect::from_corners(level_position, level_position + level_size).contains(location)
        })
        .map(|(_transform, handle)| handle.clone()) //level.identifier().to_string())
        .collect()
}

#[allow(missing_docs)]
pub type IntGridAtLocationQuery<'a, 'b> = Query<'a, 'b, (&'static GlobalTransform, &'static Layer)>;

/// Finds the int grid value, if any, at a given global location
pub fn int_grid_at_location(
    location: Vec2,
    // projects: &Assets<ProjectAsset>,
    // worlds: &Assets<WorldAsset>,
    // _levels: &Assets<LevelAsset>,
    int_grid_query: IntGridAtLocationQuery,
) -> Option<ldtk::IntGridValueDefinition> {
    // let mut level_handles = int_grid_query
    //     .iter()
    //     .map(|(transform, handle)| (transform, levels.get(handle).expect("bad level handle!")))
    //     .filter(|(transform, level)| {
    //         let level_position = transform.translation().truncate();
    //         let level_size = Vec2::new(1.0, -1.0) * level.size();
    //         Rect::from_corners(level_position, level_position + level_size).contains(location)
    //     })
    //     .collect::<Vec<_>>();
    //
    // level_handles.sort_by(|(_, a), (_, b)| {
    //     a.world_depth()
    //         .partial_cmp(&b.world_depth())
    //         .expect("bad sort?")
    // });
    //
    // level_handles.iter().rev().find(|(transform, level)| {
    //     let location_in_level = location - transform.translation().truncate();
    //     // let row = (level_location.x.floor() as usize);
    //     // let col = (level_location.y.floor() as usize);
    //     // if row > level.
    //     todo!()
    // });

    let mut layers: Vec<_> = int_grid_query
        .iter()
        .filter(|(transform, layer)| {
            let layer_position = transform.translation().truncate();
            let layer_size = Vec2::new(1.0, -1.0) * layer.size();
            Rect::from_corners(layer_position, layer_position - layer_size).contains(location)
        })
        .collect();

    layers.sort_by(|(a, _), (b, _)| {
        a.translation()
            .z
            .partial_cmp(&b.translation().z)
            .expect("bad layer sort?")
    });

    layers.iter().rev().find(|(transform, layer)| {
        let location_in_layer = location - transform.translation().truncate();
        todo!()
    });

    todo!()
}
