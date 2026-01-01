use std::path::PathBuf;

pub struct RenameItem {
    pub old_name: String,
    pub new_name: String,
    pub selected: bool,
    pub path: PathBuf,
}
