use std::path::Path;

pub struct Validate {}

impl Validate {
    pub fn is_valid_path(path: String) -> bool {
        return Path::new(&path).exists();
    }
}
