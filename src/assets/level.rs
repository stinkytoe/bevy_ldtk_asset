use bevy::{
    prelude::*,
    render::{mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology},
    sprite::{Material2d, MaterialMesh2dBundle},
};

use crate::{
    bundles::LayerBundle,
    ldtk,
    traits::{HasIdentifier, SpawnsEntities},
    util::bevy_color_from_ldtk,
};

use super::{project::ProjectAsset, world::WorldAsset};

/// An asset representing an LDTK level
#[derive(Asset, Clone, Debug, TypePath)]
pub struct LevelAsset {
    // Handle to the project which defines this level
    project_handle: Handle<ProjectAsset>,
    // reimports and remaps of ldtk objects
    background_color: Color,
    background: Option<(String, ldtk::LevelBackgroundPosition)>,
    neighbors: Vec<ldtk::NeighbourLevel>,
    field_instances: Vec<ldtk::FieldInstance>,
    identifier: String,
    iid: String,
    layer_instances: Vec<ldtk::LayerInstance>,
    size: Vec2,
    uid: i64,
    world_depth: i64,
    world_location: Vec2,
    // settings for our implementation
    layer_separation: f32,
}

impl LevelAsset {
    pub(crate) fn new(level: &ldtk::Level, project_handle: Handle<ProjectAsset>) -> Self {
        Self {
            project_handle,
            background_color: bevy_color_from_ldtk(&level.bg_color)
                .expect("bad or missing background color?"),
            // background: None,
            background: if let (Some(bg_rel_path), Some(bg_pos)) =
                (level.bg_rel_path.as_ref(), level.bg_pos.as_ref())
            {
                Some((bg_rel_path.clone(), bg_pos.clone()))
            } else {
                None
            },
            neighbors: level.neighbours.clone(),
            field_instances: level.field_instances.clone(),
            identifier: level.identifier.clone(),
            iid: level.iid.clone(),
            layer_instances: level
                .layer_instances
                .as_ref()
                .map_or_else(Vec::new, |layer_instances| {
                    layer_instances.iter().rev().cloned().collect()
                }),
            size: Vec2::new(level.px_wid as f32, level.px_hei as f32),
            uid: level.uid,
            world_depth: level.world_depth,
            world_location: Vec2::new(level.world_x as f32, -level.world_y as f32),
            layer_separation: f32::EPSILON,
        }
    }

    /// Returns a handle to the project which defines this level
    pub fn project_handle(&self) -> &Handle<ProjectAsset> {
        &self.project_handle
    }

    /// The background color of a level
    /// If a pixel is transparent in all layers and the background
    /// image, then this is the color which will show
    pub fn background_color(&self) -> Color {
        self.background_color
    }

    /// An optional background image to show, behind all layers
    pub fn background(&self) -> Option<&(String, ldtk::LevelBackgroundPosition)> {
        self.background.as_ref()
    }

    /// Returns the neighbors as referenced in the LDtk project.
    ///
    /// This only applies to certain layouts. See [__neighbors](https://ldtk.io/json/#ldtk-LevelJson;__neighbours)
    /// in the LDtk documentation
    pub fn neighbors(&self) -> &[ldtk::NeighbourLevel] {
        self.neighbors.as_ref()
    }

    /// Field instances for this level.
    ///
    /// See the LDtk documentation for [fieldInstances](https://ldtk.io/json/#ldtk-FieldInstanceJson)
    pub fn field_instances(&self) -> &[ldtk::FieldInstance] {
        self.field_instances.as_ref()
    }

    /// A unique identifier for this level.
    ///
    /// Since .identifier is also unique, we tend to use that as the unique identifer
    /// in this library.
    pub fn iid(&self) -> &str {
        self.iid.as_ref()
    }

    /// A vector of layer instance objects.
    ///
    /// See the LDtk documentation for [LayerInstance](https://ldtk.io/json/#ldtk-LayerInstanceJson).
    pub fn layer_instances(&self) -> &[ldtk::LayerInstance] {
        self.layer_instances.as_ref()
    }

    /// The size of the level in pixels.
    ///
    /// This is a remap of LDtk's [pxWid](https://ldtk.io/json/#ldtk-LevelJson;pxWid) and
    /// [pxHei](https://ldtk.io/json/#ldtk-LevelJson;pxHei) fields, casted to f32 for convenience
    /// with the Bevy engine.
    pub fn size(&self) -> Vec2 {
        self.size
    }

    /// The location in the world where this level is located.
    ///
    /// This is based on the [worldX](https://ldtk.io/json/#ldtk-LevelJson;worldX) and
    /// [worldY](https://ldtk.io/json/#ldtk-LevelJson;worldY) fields from LDtk,
    /// but converted to f32 for convenience with the Bevy engine.
    ///
    /// Also, we invert the y around the origin, to mathch Bevy's world space convention.
    pub fn world_location(&self) -> Vec2 {
        self.world_location
    }

    /// Relative depth of the level in relation to its world
    pub fn world_depth(&self) -> i64 {
        self.world_depth
    }

    /// The unique identifier from the LDtk project. (Not really used outside of the editor)
    pub fn uid(&self) -> i64 {
        self.uid
    }
}

impl HasIdentifier for LevelAsset {
    fn identifier(&self) -> &String {
        &self.identifier
    }
}

impl SpawnsEntities for LevelAsset {
    fn spawn_entities(
        &self,
        commands: &mut Commands,
        entity: Entity,
        _asset_server: &AssetServer,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
        images: &Assets<Image>,
        projects: &Assets<ProjectAsset>,
        _worlds: &Assets<WorldAsset>,
        _levels: &Assets<LevelAsset>,
    ) {
        commands
            .entity(entity)
            .insert(Name::from(self.identifier().as_str()))
            .insert(SpatialBundle {
                transform: Transform::from_translation(self.world_location.extend(0.0)),
                ..default()
            })
            .with_children(|parent| {
                let project = projects
                    .get(self.project_handle.clone())
                    .expect("couldn't get project?");

                parent.spawn(self.spawn_bg_poly(meshes, materials));

                if let Some(bundle) = self.spawn_bg_image(project, meshes, materials, images) {
                    parent.spawn(bundle);
                };
            });

        let _ = self.layer_separation;
    }
}

impl LevelAsset {
    #[allow(clippy::too_many_arguments)]
    fn spawn_generic_layer<M: Material2d + Default>(
        &self,
        meshes: &mut Assets<Mesh>,
        name: &str,
        material: impl Into<Handle<M>>,
        top_left: Vec2,
        size: Vec2,
        z: f32,
        scale: Vec2,
        uv_start: Vec2,
        uv_end: Vec2,
    ) -> LayerBundle<M> {
        let indices = Indices::U32(vec![0, 1, 2, 0, 2, 3]);
        let verts = vec![
            [0.0, 0.0, 0.0],
            [size.x, 0.0, 0.0],
            [size.x, -size.y, 0.0],
            [0.0, -size.y, 0.0],
        ];
        let uvs = vec![
            [uv_start.x, uv_start.y], //0
            [uv_end.x, uv_start.y],   //1
            [uv_end.x, uv_end.y],     //2
            [uv_start.x, uv_end.y],   //3
        ];

        LayerBundle {
            name: Name::from(name),
            mesh: MaterialMesh2dBundle {
                mesh: meshes
                    .add(
                        Mesh::new(
                            PrimitiveTopology::TriangleList,
                            RenderAssetUsages::default(),
                        )
                        .with_inserted_indices(indices)
                        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, verts)
                        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs),
                    )
                    .into(),
                material: material.into(),
                transform: Transform {
                    translation: top_left.extend(z),
                    scale: scale.extend(0.0),
                    ..default()
                },
                ..default()
            },
        }
    }

    fn spawn_bg_poly(
        &self,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
    ) -> LayerBundle<ColorMaterial> {
        self.spawn_generic_layer(
            meshes,
            "background_color",
            materials.add(ColorMaterial {
                color: self.background_color,
                texture: None,
            }),
            Vec2::ZERO,
            self.size,
            0.0,
            Vec2::ONE,
            Vec2::ZERO,
            Vec2::ONE,
        )
    }

    fn spawn_bg_image(
        &self,
        project: &ProjectAsset,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
        images: &Assets<Image>,
    ) -> Option<LayerBundle<ColorMaterial>> {
        if let Some((path, bg_pos)) = &self.background {
            let handle = project.backgrounds.get(path).expect("bad image path?");
            let image = images.get(handle).expect("bad handle?");
            let image_size = image.size_f32();
            let (crop_x, crop_y, crop_width, crop_height) = (
                bg_pos.crop_rect[0] as f32,
                bg_pos.crop_rect[1] as f32,
                bg_pos.crop_rect[2] as f32,
                bg_pos.crop_rect[3] as f32,
            );

            Some(self.spawn_generic_layer(
                meshes,
                "background_image",
                materials.add(ColorMaterial {
                    color: Color::WHITE,
                    texture: Some(handle.clone()),
                }),
                Vec2::new(bg_pos.top_left_px[0] as f32, -bg_pos.top_left_px[1] as f32),
                Vec2::new(crop_width, crop_height),
                self.layer_separation,
                Vec2::new(bg_pos.scale[0] as f32, bg_pos.scale[1] as f32),
                Vec2::new(crop_x / image_size.x, crop_y / image_size.y),
                Vec2::new(
                    (crop_x + crop_width) / image_size.x,
                    (crop_y + crop_height) / image_size.y,
                ),
            ))
        } else {
            None
        }
    }
}
