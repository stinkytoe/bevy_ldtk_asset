use std::path::Path;

pub fn ldtk_file_to_asset_path(ldtk_file: &str, ldtk_path: &Path) -> String {
    let path = ldtk_path.parent().unwrap().join(ldtk_file);
    path.to_str().unwrap().to_string()
}
