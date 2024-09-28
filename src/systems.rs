#[cfg(feature = "asset_events_debug")]
use crate::{prelude::LdtkAsset, project::Project};
#[cfg(feature = "asset_events_debug")]
use bevy::{asset::AssetEvent, log::debug, prelude::EventReader};

#[cfg(feature = "asset_events_debug")]
pub(crate) fn project_debug_output(mut project_events: EventReader<AssetEvent<Project>>) {
    project_events
        .read()
        .for_each(|event| debug!("AssetEvent: {event:?}"));
}

#[cfg(feature = "asset_events_debug")]
pub(crate) fn ldtk_asset_debug_output<A: LdtkAsset>(mut events: EventReader<AssetEvent<A>>) {
    events
        .read()
        .for_each(|event| debug!("AssetEvent: {event:?}"));
}
