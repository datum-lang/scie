#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RawLanguageExt {
    id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    extensions: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    filenames: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(alias = "filenamePatterns")]
    filename_patterns: Option<Vec<String>>,

    #[serde(alias = "firstLine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    first_line: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    aliases: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    mimetypes: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    configuration: Option<String>,
}