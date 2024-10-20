#[cfg(feature = "asset_events_debug")]
pub(crate) mod asset_events_debug {
    use bevy::asset::AssetEvent;
    use bevy::log::debug;
    use bevy::prelude::EventReader;

    use crate::ldtk_asset_traits::{HasIdentifier, LdtkAsset};
    use crate::project::Project;

    pub(crate) fn ldtk_project_debug_output(mut project_events: EventReader<AssetEvent<Project>>) {
        project_events
            .read()
            .for_each(|event| debug!("AssetEvent: {event:?}"));
    }

    pub(crate) fn ldtk_asset_debug_output<A: LdtkAsset + HasIdentifier>(
        mut events: EventReader<AssetEvent<A>>,
    ) {
        events
            .read()
            .for_each(|event| debug!("AssetEvent: {event:?}"));
    }
}
