#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RawLanguageExt {
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub filenames: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(alias = "filenamePatterns")]
    pub filename_patterns: Option<Vec<String>>,

    #[serde(alias = "firstLine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_line: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mimetypes: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,
}
