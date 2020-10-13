use walkdir::WalkDir;
use std::collections::{BTreeMap, HashSet};
use std::collections::hash_map::RandomState;

pub struct Framework {
    pub name: String,
    pub path: String,
    // for find the projects
    pub relative_path: String,
    // in some languages has different framework file
    // |   languages |   files    |
    // |-------------|------------|
    // | Java        | build.gradle, settings.gradle |
    pub framework_files: Vec<String>,
    // in JVM projects, has different languages, such as Java, Groovy, Kotlin...
    pub language: Vec<String>,
}

pub struct FrameworkDetector {
    pub tags: BTreeMap<String, bool>,
    pub names: Vec<String>,
    pub frameworks: Vec<Framework>,
}

impl FrameworkDetector {
    pub fn new() -> Self {
        FrameworkDetector {
            tags: Default::default(),
            names: vec![],
            frameworks: vec![],
        }
    }

    pub fn run(&self, path: String) {
        FrameworkDetector::light_detector(path)
    }

    fn light_detector(path: String) {
        let name_set = FrameworkDetector::build_level_one_name_set(path);
        name_set.contains("build.gradle");
    }

    pub fn build_level_one_name_set(path: String) -> HashSet<String, RandomState> {
        let mut name_sets: HashSet<String> = HashSet::new();
        let walk_dir = WalkDir::new(path);
        for dir_entry in walk_dir.max_depth(1).into_iter() {
            if dir_entry.is_err() {
                continue;
            }

            let entry = dir_entry.unwrap();
            let file_name = entry.path().file_name().unwrap().clone();
            name_sets.insert(file_name.to_str().unwrap().to_string());
        }

        name_sets
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::framework_detector::FrameworkDetector;

    #[test]
    fn it_works() {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();

        let test_project_dir = root_dir.clone()
            .join("fixtures")
            .join("projects")
            .join("java")
            .join("simple");

        // let detector = FrameworkDetector::new();
        // detector.run(test_project_dir.display().to_string());

        let name_set = FrameworkDetector::build_level_one_name_set(test_project_dir.display().to_string());
        assert!(name_set.contains("build.gradle"));
    }
}

