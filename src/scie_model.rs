pub struct ScieModel {
    pub file_name: String,
    pub path: String,
    pub line: i32,
    pub start_index: i32,
    pub end_index: i32,
    pub scopes: Vec<String>,
}