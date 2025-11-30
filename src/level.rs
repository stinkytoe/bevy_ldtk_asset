//! The LDtk level, represented as a Bevy asset.
//!
//! This is an import of an LDtk
//! [LevelInstance](https://ldtk.io/json/#ldtk-LevelInstanceJson).

use std::str::FromStr;
use std::sync::{Arc, RwLock};

use bevy_asset::{Asset, Handle, LoadContext};
use bevy_color::Color;
use bevy_image::Image;
use bevy_log::debug;
use bevy_math::{DVec2, I64Vec2};
use bevy_platform::collections::HashMap;
use bevy_reflect::Reflect;
use futures::future::try_join_all;
use futures::lock::Mutex;

use crate::color::bevy_color_from_ldtk_string;
use crate::field_instance::FieldInstance;
use crate::iid::{Iid, IidMap};
use crate::layer::LayerInstance;
use crate::ldtk_asset_trait::{LdtkAsset, LdtkAssetWithChildren, LdtkAssetWithFieldInstances};
use crate::ldtk_path::ldtk_path_to_bevy_path;
use crate::project::ProjectContext;
use crate::result::LdtkResult;
use crate::uid::Uid;
use crate::{ldtk, ldtk_import_error};

/// Relatve direction of levels in the Neighbour list.
#[allow(missing_docs)]
#[derive(Debug, Reflect)]
pub enum NeighbourDir {
    North,
    South,
    East,
    West,
    Lower,
    Greater,
    Overlap,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

impl NeighbourDir {
    fn new(dir: &str) -> LdtkResult<Self> {
        match dir {
            "n" => Ok(Self::North),
            "s" => Ok(Self::South),
            "w" => Ok(Self::West),
            "e" => Ok(Self::East),
            "<" => Ok(Self::Lower),
            ">" => Ok(Self::Greater),
            "o" => Ok(Self::Overlap),
            "nw" => Ok(Self::NorthWest),
            "ne" => Ok(Self::NorthEast),
            "sw" => Ok(Self::SouthWest),
            "se" => Ok(Self::SouthEast),
            _ => Err(ldtk_import_error!(
                "Bad direction from LDtk neighbor! given: {dir}"
            )),
        }
    }
}

/// An entry in the list of Neighbours, indicating the Iid of the adjacent level, and its relative
/// direction.
#[allow(missing_docs)]
#[derive(Debug, Reflect)]
pub struct Neighbour {
    pub dir: NeighbourDir,
    pub level_iid: Iid,
}

impl Neighbour {
    pub(crate) fn new(value: ldtk::NeighbourLevel) -> LdtkResult<Self> {
        let dir = NeighbourDir::new(&value.dir)?;
        let level_iid = Iid::from_str(&value.level_iid)?;

        Ok(Self { dir, level_iid })
    }
}

/// The background of the level. This is to be drawn below the associated layers.
/// [LevelBackground::crop_corner], and [LevelBackground::crop_size] represent the region inside of
/// the image that should be cropped out for use in the visualization of the [Layer].
///
/// [LevelBackground::corner] is relative to the top left corner of the level, and
/// [LevelBackground::scale] is a scale factor which should be applied to the visualiztion.
///
/// Even though crop_corner, and crop_size represent pixel space locations, they are given to us as
/// f64 from LDtk so we will also pass them on as f64 values.
#[allow(missing_docs)]
#[derive(Clone, Debug, Reflect)]
pub struct LevelBackground {
    pub image: Handle<Image>,
    pub crop_corner: DVec2,
    pub crop_size: DVec2,
    pub scale: DVec2,
    pub corner: I64Vec2,
}

impl LevelBackground {
    pub(crate) fn new(
        value: ldtk::LevelBackgroundPosition,
        image: Handle<Image>,
    ) -> LdtkResult<Self> {
        let (crop_corner, crop_size) = (value.crop_rect.len() == 4)
            .then(|| {
                let crop_corner = (value.crop_rect[0], value.crop_rect[1]).into();
                let crop_size = (value.crop_rect[2], value.crop_rect[3]).into();
                (crop_corner, crop_size)
            })
            .ok_or(ldtk_import_error!(
                "Bad value for crop! given: {:?}",
                value.crop_rect
            ))?;
        let scale = (value.scale.len() == 2)
            .then(|| (value.scale[0], value.scale[1]).into())
            .ok_or(ldtk_import_error!(
                "Bad value for scale! given: {:?}",
                value.crop_rect
            ))?;
        let corner = (value.top_left_px.len() == 2)
            .then(|| (value.top_left_px[0], value.top_left_px[1]).into())
            .ok_or(ldtk_import_error!(
                "Bad value for corner! given: {:?}",
                value.crop_rect
            ))?;

        Ok(Self {
            image,
            crop_corner,
            crop_size,
            scale,
            corner,
        })
    }
}

/// A level as represented in an LDtk project.
///
/// See [LevelInstance](https://ldtk.io/json/#ldtk-LevelInstanceJson).
#[derive(Debug, Asset, Reflect)]
pub struct Level {
    /// The background color. This should represent a rectangle exactly the size and location of the
    /// [Level], represented by [Level::location] and [Level::size]. It should be drawn 'behind'
    /// this level's associated [crate::layer::Layer]s, if any.
    pub bg_color: Color,
    /// A list of [Neighbour]s, representing adjacent levels.
    pub neighbours: Vec<Neighbour>,
    /// An optional level background image. If not present, the level's visualization is supposed
    /// to be a square of color [Level::bg_color]. If this is present, then the image defined here
    /// should be overlayed onto the colored square.
    pub background: Option<LevelBackground>,
    /// The [crate::field_instance::FieldInstance]s associated with this Level.
    pub field_instances: HashMap<String, FieldInstance>,
    /// A unique string representing this level.
    pub identifier: String,
    /// A unique [Iid] representing this level.
    pub iid: Iid,
    /// The size, in pixels, of this Level. In LDtk, all layer visualizations are cropped to fit
    /// within this region.
    pub size: I64Vec2,
    /// A soon to be deprecated value from LDtk. Added here for completeness, but likely to be
    /// removed in the future.
    pub uid: Uid, // TODO: do we need this?
    /// An integer representing the stacking of Levels in a levels given world. Positive world
    /// depths are meant to be visualized 'above' lower value levels.
    pub world_depth: i64,
    /// The relative location of this [Level] within its associated [crate::world::World].
    ///
    /// This is in LDtk's coordinate space.
    pub location: I64Vec2,
    /// Handles to all of the associated [Layer] instances, indexed by that layer's [Iid].
    ///
    /// NOTE: There is no meaning to the order within this field. If the order of the layers is
    /// needed, the [Layer::index] field represents the order of  the layer within the set.
    pub layers: IidMap<Handle<LayerInstance>>,
    /// The unique index of this level.
    ///
    /// This only has meaning for
    /// [crate::world::WorldLayout::LinearVertical] and
    /// [crate::world::WorldLayout::LinearHorizontal] world layouts.
    pub index: usize,
}

impl Level {
    pub(crate) async fn new(
        level_json: ldtk::Level,
        index: usize,
        project_context: Arc<RwLock<ProjectContext>>,
        load_context: Arc<Mutex<&mut LoadContext<'_>>>,
        level_label: &str,
    ) -> LdtkResult<Self> {
        let identifier = level_json.identifier;

        let bg_color = bevy_color_from_ldtk_string(&level_json.bg_color)?;
        let neighbours = level_json
            .neighbours
            .into_iter()
            .map(Neighbour::new)
            .collect::<LdtkResult<_>>()?;

        let background = match (level_json.bg_pos, level_json.bg_rel_path) {
            (None, None) => None,
            (None, Some(_)) => {
                return Err(ldtk_import_error!(
                    "bg_pos is None while bg_rel_path is Some(_)!"
                ));
            }
            (Some(_), None) => {
                return Err(ldtk_import_error!(
                    "bg_pos is Some(_) while bg_rel_path is None!"
                ));
            }
            (Some(bg_pos), Some(bg_rel_path)) => {
                let path = ldtk_path_to_bevy_path(
                    project_context.read()?.project_directory.as_path(),
                    bg_rel_path,
                );
                let image = load_context.lock().await.load(path);
                let background = LevelBackground::new(bg_pos, image)?;
                Some(background)
            }
        };

        let iid = Iid::from_str(&level_json.iid)?;

        let field_instances_iter = level_json
            .field_instances
            .into_iter()
            .filter(|field_instance_json| field_instance_json.value.is_some())
            .map(|field_instance_json| {
                let project_context = project_context.clone();
                async move {
                    let identifier = field_instance_json.identifier.clone();
                    let field_instance =
                        FieldInstance::new(field_instance_json, project_context).await?;

                    LdtkResult::Ok((identifier, field_instance))
                }
            });

        let field_instances = try_join_all(field_instances_iter)
            .await?
            .into_iter()
            .collect();

        let size = (level_json.px_wid, level_json.px_hei).into();

        let uid = level_json.uid;

        let world_depth = level_json.world_depth;

        let location = (level_json.world_x, level_json.world_y).into();

        let layer_instances = level_json.layer_instances.ok_or(ldtk_import_error!(
            "layer_instances is None? Are we opening the local layer definition instead of the external one?"
        ))?;

        let num_layers = layer_instances.len();

        let layers_iter =
            layer_instances
                .into_iter()
                .enumerate()
                .map(|(index, layer_instance_json)| {
                    let project_context = project_context.clone();
                    let load_context = load_context.clone();
                    let index = num_layers - index - 1;

                    async move {
                        let layer_label =
                            format!("{level_label}/{}", layer_instance_json.identifier);
                        debug!("constructing layer asset: {layer_label}");

                        let iid = Iid::parse_str(&layer_instance_json.iid)?;

                        let layer = LayerInstance::new(
                            layer_instance_json,
                            index,
                            project_context,
                            load_context.clone(),
                            &layer_label,
                        )
                        .await?;

                        let handle = load_context
                            .lock()
                            .await
                            .add_labeled_asset(layer_label, layer);

                        LdtkResult::Ok((iid, handle))
                    }
                });

        let layers = try_join_all(layers_iter).await?.into_iter().collect();

        Ok(Self {
            bg_color,
            neighbours,
            background,
            identifier,
            iid,
            field_instances,
            size,
            uid,
            world_depth,
            location,
            layers,
            index,
        })
    }
}

impl LdtkAsset for Level {
    fn get_identifier(&self) -> &str {
        &self.identifier
    }

    fn get_iid(&self) -> Iid {
        self.iid
    }
}

impl LdtkAssetWithChildren<LayerInstance> for Level {
    fn get_children(&self) -> impl Iterator<Item = &Handle<LayerInstance>> {
        self.layers.values()
    }
}

impl LdtkAssetWithFieldInstances for Level {
    fn get_field_instance(&self, identifier: &str) -> Option<&FieldInstance> {
        self.field_instances.get(identifier)
    }
}
