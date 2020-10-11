#[macro_use]
extern crate serde_derive;

extern crate serde;

use walkdir::{WalkDir};
use std::path::{PathBuf, Display, Path};
use scie_infra::finder::Finder;
use scie_model::JsonPackage;

pub fn walk_dir(path: String) -> Vec<PathBuf> {
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

fn main() {
    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf();
    let ext_path = root_dir.join("extensions");
    let package_files = walk_dir(ext_path.to_str().unwrap().to_string());
    for path in package_files {
        let package = Finder::read_code(&path);
        let pkg: JsonPackage = serde_json::from_str(&package).unwrap();
    }
}

#[cfg(test)]
mod tests {

}
