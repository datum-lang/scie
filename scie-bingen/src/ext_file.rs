use std::path::PathBuf;
use walkdir::WalkDir;

pub struct ExtFile {

}

impl ExtFile {
    pub(crate) fn walk_dir(path: String) -> Vec<PathBuf> {
        let mut packages = vec![];
        let walk_dir = WalkDir::new(path);

        let filtered_entries = walk_dir.max_depth(2).into_iter();
        for entry in filtered_entries {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.display().to_string().ends_with("package.json") {
                packages.push(path.to_path_buf());
            }
        }

        packages
    }
}