#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ContentRoot {
    pub root_dirs: Vec<String>,
    pub source_dirs: Vec<String>,
    pub gen_source_dirs: Vec<String>,
    pub resource_dirs: Vec<String>,
    pub test_source_dirs: Vec<String>,
    pub get_test_dirs: Vec<String>,
    pub get_test_source_dirs: Vec<String>,
    pub exclude_dirs: Vec<String>,
}
