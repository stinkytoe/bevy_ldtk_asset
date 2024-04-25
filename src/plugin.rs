use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

use crate::layer::LayerPlugin;
use crate::level::LevelPlugin;
use crate::project::ProjectPlugin;
use crate::world::WorldPlugin;

#[derive(Debug, Default)]
pub struct LdtkLevelsPlugins;

impl PluginGroup for LdtkLevelsPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(ProjectPlugin)
            .add(WorldPlugin)
            .add(LevelPlugin)
            .add(LayerPlugin)
    }
}
