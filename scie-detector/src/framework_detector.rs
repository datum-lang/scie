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
    pub frameworks: Vec<Framework>
}

impl FrameworkDetector {
    pub fn new() -> Self {
        FrameworkDetector {
            names: vec![],
            frameworks: vec![]
        }
    }

    pub fn run() {

    }
}

