use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::utils::error;
use thiserror::Error;

use crate::field_instance::FieldInstance;
use crate::field_instance::FieldInstanceValueParseError;
use crate::ldtk;
use crate::project::ProjectAsset;
use crate::tileset_rectangle::TilesetRectangle;
use crate::util::bevy_anchor_from_ldtk;
use crate::util::bevy_color_from_ldtk;
use crate::util::AnchorIntoError;
use crate::util::ColorParseError;

#[derive(Debug, Error)]
pub enum EntityComponentError {
    #[error("AnchorIntoError {0}")]
    AnchorIntoError(#[from] AnchorIntoError),
    #[error("ColorParseError {0}")]
    ColorParseError(#[from] ColorParseError),
    #[error("WorldCoordMixedOptionError")]
    WorldCoordMixedOptionError,
    #[error("FieldInstanceValueError {0}")]
    FieldInstanceValueError(#[from] FieldInstanceValueParseError),
}

#[derive(Component, Debug)]
#[cfg_attr(feature = "enable_reflect", derive(Reflect))]
pub struct EntityComponent {
    grid: IVec2,
    identifier: String,
    anchor: Anchor,
    smart_color: Color,
    tags: Vec<String>,
    tile: Option<TilesetRectangle>,
    world_location: Option<Vec2>,
    def_uid: i64,
    field_instances: Vec<FieldInstance>,
    size: Vec2,
    iid: String,
    location: Vec2,
}

impl EntityComponent {
    pub fn grid(&self) -> IVec2 {
        self.grid
    }

    pub fn identifier(&self) -> &str {
        self.identifier.as_ref()
    }

    pub fn anchor(&self) -> Anchor {
        self.anchor
    }

    pub fn smart_color(&self) -> Color {
        self.smart_color
    }

    pub fn tags(&self) -> &[String] {
        self.tags.as_ref()
    }

    pub fn tile(&self) -> Option<&TilesetRectangle> {
        self.tile.as_ref()
    }

    pub fn world_location(&self) -> Option<Vec2> {
        self.world_location
    }

    pub fn def_uid(&self) -> i64 {
        self.def_uid
    }

    pub fn field_instances(&self) -> &[FieldInstance] {
        self.field_instances.as_ref()
    }

    pub fn size(&self) -> Vec2 {
        self.size
    }

    pub fn iid(&self) -> &str {
        self.iid.as_ref()
    }

    pub fn location(&self) -> Vec2 {
        self.location
    }
}

impl EntityComponent {
    pub fn has_tag(&self, tag: &str) -> bool {
        // using .iter().any(...) instead of .contains(...) to avoid allocations
        self.tags.iter().any(|inner_tag| inner_tag == tag)
    }

    pub fn get_field_instance_by_identifier(&self, identifier: &str) -> Option<&FieldInstance> {
        self.field_instances
            .iter()
            .find(|field_instance| field_instance.identifier() == identifier)
    }
}

impl TryFrom<&ldtk::EntityInstance> for EntityComponent {
    type Error = EntityComponentError;

    fn try_from(value: &ldtk::EntityInstance) -> Result<Self, Self::Error> {
        Ok(Self {
            grid: (value.grid[0] as i32, value.grid[1] as i32).into(),
            identifier: value.identifier.clone(),
            anchor: bevy_anchor_from_ldtk(&value.pivot)?,
            smart_color: bevy_color_from_ldtk(&value.smart_color)?,
            tags: value.tags.clone(),
            tile: value
                .tile
                .as_ref()
                .map(|tileset_rectangle| tileset_rectangle.into()),
            world_location: match (value.world_x, value.world_y) {
                (None, None) => None,
                (Some(world_x), Some(world_y)) => Some((world_x as f32, world_y as f32).into()),
                (None, Some(_)) | (Some(_), None) => {
                    return Err(EntityComponentError::WorldCoordMixedOptionError)
                }
            },
            def_uid: value.def_uid,
            field_instances: value
                .field_instances
                .iter()
                .map(|field_instance| field_instance.try_into())
                .collect::<Result<_, _>>()?,
            size: (value.width as f32, value.height as f32).into(),
            iid: value.iid.clone(),
            location: (value.px[0] as f32, value.px[1] as f32).into(),
        })
    }
}

#[derive(Debug, Default)]
pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_systems(
                Update,
                (
                    tileset_rectangle_added_in_entity.map(error),
                    tileset_rectangle_changed_in_entity.map(error),
                ),
            );

        #[cfg(feature = "enable_reflect")]
        {
            app //
                .register_type::<EntityComponent>();
        }
    }
}

#[derive(Debug, Error)]
pub(crate) enum EntityComponentChangedTilesetRectangleError {
    #[error("Bad project handle!")]
    BadProjectHandle,
    #[error("Bad tileset definition uid!")]
    BadTilesetDefinitionUid(i64),
    #[error("EntityComponentError: {0}")]
    EntityComponentError(#[from] EntityComponentError),
    #[error("No tileset path?")]
    MissingTilesetPath,
    #[error("BadTilesetPath")]
    BadTilesetPath,
}

pub(crate) fn tileset_rectangle_added_in_entity(
    mut commands: Commands,
    project_assets: Res<Assets<ProjectAsset>>,
    with_sprite_query: Query<
        (
            Entity,
            &Handle<ProjectAsset>,
            &EntityComponent,
            &TilesetRectangle,
        ),
        Added<TilesetRectangle>,
    >,
) -> Result<(), EntityComponentChangedTilesetRectangleError> {
    for (entity, project_handle, entity_component, tileset_rectangle) in with_sprite_query.iter() {
        debug!(
            "TilesetRectangle added for: {}!",
            entity_component.identifier()
        );
        let project_asset = project_assets
            .get(project_handle)
            .ok_or(EntityComponentChangedTilesetRectangleError::BadProjectHandle)?;

        let tileset_definition = project_asset
            .get_tileset_definition_by_uid(tileset_rectangle.tileset_uid())
            .ok_or(
                EntityComponentChangedTilesetRectangleError::BadTilesetDefinitionUid(
                    tileset_rectangle.tileset_uid(),
                ),
            )?;

        let color = Color::WHITE;

        let custom_size = Some(tileset_rectangle.size());

        let rect = Some(Rect::from_corners(
            tileset_rectangle.location(),
            tileset_rectangle.location() + tileset_rectangle.size(),
        ));

        let anchor = entity_component.anchor();

        let texture = project_asset
            .get_tileset_handle(
                tileset_definition
                    .rel_path
                    .as_ref()
                    .ok_or(EntityComponentChangedTilesetRectangleError::MissingTilesetPath)?,
            )
            .ok_or(EntityComponentChangedTilesetRectangleError::BadTilesetPath)?
            .clone();

        let sprite = Sprite {
            color,
            custom_size,
            rect,
            anchor,
            ..default()
        };

        commands
            .entity(entity)
            .remove::<(Handle<Image>, Sprite)>()
            .insert((texture, sprite));
    }

    Ok(())
}

#[allow(clippy::type_complexity)]
pub(crate) fn tileset_rectangle_changed_in_entity(
    project_assets: Res<Assets<ProjectAsset>>,
    mut with_sprite_query: Query<
        (
            &Handle<ProjectAsset>,
            &EntityComponent,
            &TilesetRectangle,
            &mut Handle<Image>,
            &mut Sprite,
        ),
        Changed<TilesetRectangle>,
    >,
) -> Result<(), EntityComponentChangedTilesetRectangleError> {
    for (project_handle, entity_component, tileset_rectangle, mut image_handle, mut sprite) in
        with_sprite_query.iter_mut()
    {
        debug!(
            "TilesetRectangle changed for: {}!",
            entity_component.identifier()
        );

        let project_asset = project_assets
            .get(project_handle)
            .ok_or(EntityComponentChangedTilesetRectangleError::BadProjectHandle)?;

        let tileset_definition = project_asset
            .get_tileset_definition_by_uid(tileset_rectangle.tileset_uid())
            .ok_or(
                EntityComponentChangedTilesetRectangleError::BadTilesetDefinitionUid(
                    tileset_rectangle.tileset_uid(),
                ),
            )?;

        let custom_size = Some(tileset_rectangle.size());

        let rect = Some(Rect::from_corners(
            tileset_rectangle.location(),
            tileset_rectangle.location() + tileset_rectangle.size(),
        ));

        let texture = project_asset
            .get_tileset_handle(
                tileset_definition
                    .rel_path
                    .as_ref()
                    .ok_or(EntityComponentChangedTilesetRectangleError::MissingTilesetPath)?,
            )
            .ok_or(EntityComponentChangedTilesetRectangleError::BadTilesetPath)?
            .clone();

        *image_handle = texture;
        sprite.custom_size = custom_size;
        sprite.rect = rect;
    }

    Ok(())
}
