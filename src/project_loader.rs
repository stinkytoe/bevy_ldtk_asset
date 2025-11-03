use bevy_asset::AssetLoader;

// use crate::LdtkResult;
// use crate::asset_labels::ProjectAssetPath;
// use crate::entity_definition::EntityDefinition;
// use crate::enum_definition::EnumDefinition;
// use crate::layer_definition::LayerDefinition;
use crate::ldtk;
use crate::project::Project;
use crate::result::Result;
// use crate::world::World;

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
