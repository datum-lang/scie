use walkdir::WalkDir;

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
    pub names: Vec<String>,
    pub frameworks: Vec<Framework>,
}

impl FrameworkDetector {
    pub fn new() -> Self {
        FrameworkDetector {
            names: vec![],
            frameworks: vec![],
        }
    }

    pub fn run(&self, path: String) {
        let walk_dir = WalkDir::new(path);
        for path in walk_dir.max_depth(1).into_iter() {
            println!("{:?}", path);
        }
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

        let detector = FrameworkDetector::new();
        detector.run(test_project_dir.display().to_string());
    }
}

