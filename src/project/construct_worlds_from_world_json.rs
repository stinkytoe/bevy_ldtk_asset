use std::sync::{Arc, RwLock};

use bevy_asset::{Handle, LoadContext};
use bevy_log::debug;
use futures::future::try_join_all;
use futures::lock::Mutex;

use crate::iid::IidMap;
use crate::ldtk;
use crate::project::ProjectContext;
use crate::result::LdtkResult;
use crate::world::World;

pub(super) async fn construct_worlds_from_world_json(
    worlds_json: IidMap<ldtk::World>,
    project_context: Arc<RwLock<ProjectContext>>,
    load_context: &mut LoadContext<'_>,
) -> LdtkResult<IidMap<Handle<World>>> {
    let load_context = Arc::new(Mutex::new(load_context));
    let worlds_iter = worlds_json.into_iter().map(|(iid, world_json)| {
        let world_label = format!("world:{}", world_json.identifier);
        debug!("constructing world asset: {world_label}");
        let project_context = project_context.clone();
        let load_context = load_context.clone();
        async move {
            let world = World::new(
                world_json,
                project_context,
                load_context.clone(),
                &world_label,
            )
            .await?;

            let handle = load_context
                .lock()
                .await
                .add_labeled_asset(world_label, world);

            LdtkResult::Ok((iid, handle))
        }
    });

    Ok(try_join_all(worlds_iter).await?.into_iter().collect())
}
