use std::path::Path;
use std::str::FromStr;

use bevy_asset::AssetLoader;
use bevy_asset::Handle;
use bevy_log::debug;
use bevy_platform::collections::HashMap;

// use crate::LdtkResult;
// use crate::asset_labels::ProjectAssetPath;
// use crate::entity_definition::EntityDefinition;
// use crate::enum_definition::EnumDefinition;
use crate::error::Error;
use crate::iid::Iid;
use crate::iid::IidMap;
use crate::iid::IidSet;
// use crate::layer_definition::LayerDefinition;
use crate::ldtk;
use crate::ldtk_import_error;
use crate::project::Project;
use crate::result::Result;
use crate::tileset_definition::TilesetDefinition;
use crate::uid::UidMap;
// use crate::world::World;

// pub(crate) struct UniqueIidAuditor {
//     known_iids: IidSet,
// }
//
// impl UniqueIidAuditor {
//     pub(crate) fn new() -> Self {
//         let known_iids = IidSet::default();
//
//         Self { known_iids }
//     }
//
//     pub(crate) fn check(&mut self, iid: Iid) -> Result<()> {
//         self.known_iids
//             .insert(iid)
//             .then_some(())
//             .ok_or(Error::DuplicateIidError(iid))
//     }
// }

pub(crate) struct ProjectDefinitionContext<'a> {
    pub(crate) tileset_definitions: &'a UidMap<Handle<TilesetDefinition>>,
    // pub(crate) layer_definitions: &'a UidMap<Handle<LayerDefinition>>,
    // pub(crate) entity_definitions: &'a UidMap<Handle<EntityDefinition>>,
    // pub(crate) enum_definitions: &'a HashMap<String, Handle<EnumDefinition>>,
}

#[derive(Default)]
pub(crate) struct ProjectLoader;

impl AssetLoader for ProjectLoader {
    type Asset = Project;
    type Settings = ();
    type Error = crate::error::Error;

    async fn load(
        &self,
        reader: &mut dyn bevy_asset::io::Reader,
        _settings: &Self::Settings,
        load_context: &mut bevy_asset::LoadContext<'_>,
    ) -> Result<Self::Asset> {
        let ldtk_project: ldtk::LdtkProject = {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            serde_json::from_slice(&bytes)?
        };

        let project = Project::new(ldtk_project, load_context).await?;

        Ok(project)
    }

    fn extensions(&self) -> &[&str] {
        &["ldtk"]
    }
}
