#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RawLanguageExtensionPoint {
    id: String,
    extensions: Vec<String>,
    filenames: Vec<String>,
    #[serde(alias = "filenamePatterns")]
    filename_patterns: Vec<String>,
    #[serde(alias = "firstLine")]
    first_line: String,
    aliases: Vec<String>,
    mimetypes: Vec<String>,
    configuration: String,
}