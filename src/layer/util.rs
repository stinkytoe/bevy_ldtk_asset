use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;
use image::imageops::crop_imm;
use image::imageops::flip_horizontal;
use image::imageops::flip_vertical;
use image::imageops::overlay;
use image::ColorType;
use image::DynamicImage;
use thiserror::Error;

use crate::layer::Tiles;

#[derive(Debug, Error)]
pub(crate) enum BuildImageFromTilesError {
    // TODO: Waiting on bevy 0.14
    // see https://github.com/bevyengine/bevy/pull/13223
    // let mut tileset = tileset_image.clone().try_into_dynamic()?;
    #[error("try_into_dynamic Failed!")]
    TryIntoDynamicFailed,
}

pub(crate) fn build_image_from_tiles(
    tileset: &Image,
    canvas_size: UVec2,
    tile_size: UVec2,
    tiles: &Tiles,
) -> Result<Image, BuildImageFromTilesError> {
    let tileset = tileset
        .clone()
        .try_into_dynamic()
        .map_err(|_| BuildImageFromTilesError::TryIntoDynamicFailed)?;

    let mut dynamic_image = DynamicImage::new(canvas_size.x, canvas_size.y, ColorType::Rgba8);

    tiles.tiles.iter().for_each(|tile| {
        // trace!("Tile loaded! {tile:?}");
        let mut cropped = crop_imm(
            &tileset,
            tile.source.x,
            tile.source.y,
            tile_size.x,
            tile_size.y,
        )
        .to_image();

        if tile.flip_h {
            cropped = flip_horizontal(&cropped);
        }

        if tile.flip_v {
            cropped = flip_vertical(&cropped);
        }

        overlay(
            &mut dynamic_image,
            &cropped,
            tile.location.x,
            tile.location.y,
        );
    });

    Ok(Image::from_dynamic(
        dynamic_image,
        true,
        RenderAssetUsages::default(),
    ))
}

pub(crate) fn create_tile_layer_mesh(size: Vec2) -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_indices(Indices::U32(vec![0, 1, 2, 0, 2, 3]))
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [0.0, 0.0, 0.0],
            [size.x, 0.0, 0.0],
            [size.x, -size.y, 0.0],
            [0.0, -size.y, 0.0],
        ],
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
    )
}
