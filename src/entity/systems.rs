use bevy::prelude::*;
use thiserror::Error;

use crate::entity::EntityAsset;
use crate::project::defs::TilesetRectangle;
use crate::project::ProjectAsset;

#[derive(Debug, Error)]
pub(crate) enum HandleEntitySpriteError {
    #[error("Bad entity asset handle!")]
    BadEntityAssetHandle,
    #[error("Bad project asset handle!")]
    BadProjectAssetHandle,
    #[error("Bad tileset Uid!")]
    BadTilesetUid,
    #[error("Missing tileset path?")]
    MissingTilesetPath,
    #[error("Bad tileset path?")]
    BadTilesetPath,
}

#[allow(clippy::type_complexity)]
pub(crate) fn handle_entity_sprite(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &Handle<EntityAsset>,
            Option<&TilesetRectangle>,
            Option<&mut Sprite>,
        ),
        Changed<TilesetRectangle>,
    >,
    entity_assets: Res<Assets<EntityAsset>>,
    project_assets: Res<Assets<ProjectAsset>>,
) -> Result<(), HandleEntitySpriteError> {
    for (entity, entity_asset, tile, sprite) in query.iter_mut() {
        let entity_asset = entity_assets
            .get(entity_asset)
            .ok_or(HandleEntitySpriteError::BadEntityAssetHandle)?;

        let project_asset = project_assets
            .get(&entity_asset.project)
            .ok_or(HandleEntitySpriteError::BadProjectAssetHandle)?;

        match tile {
            Some(tile) => {
                let tileset_definition = project_asset
                    .tileset_defs
                    .get(&tile.tileset_uid())
                    .ok_or(HandleEntitySpriteError::BadTilesetUid)?;

                let color = Color::WHITE;

                let custom_size = Some(tile.size());

                let rect = Some(Rect::from_corners(
                    tile.location(),
                    tile.location() + tile.size(),
                ));

                let anchor = entity_asset.anchor;

                let texture = project_asset
                    .tileset_assets
                    .get(
                        tileset_definition
                            .rel_path
                            .as_ref()
                            .ok_or(HandleEntitySpriteError::MissingTilesetPath)?,
                    )
                    .ok_or(HandleEntitySpriteError::BadTilesetPath)?
                    .clone();

                if let Some(mut sprite) = sprite {
                    sprite.color = color;
                    sprite.custom_size = custom_size;
                    sprite.rect = rect;
                    sprite.anchor = anchor;
                } else {
                    commands.entity(entity).insert(Sprite {
                        color,
                        custom_size,
                        rect,
                        anchor,
                        ..default()
                    });
                };

                commands.entity(entity).insert(texture);
            }
            None => {
                commands.entity(entity).remove::<Handle<Image>>();
                commands.entity(entity).remove::<Sprite>();
            }
        }
    }

    Ok(())
}
