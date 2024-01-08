use crate::ldtk_json;

/// A read-only object which represents the layer definition
/// as defined in the LDtk project.
pub struct LayerDefinition<'a> {
    pub(crate) _value: &'a ldtk_json::LayerDefinition,
}
