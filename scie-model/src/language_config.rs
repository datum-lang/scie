#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommentRule {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(alias = "lineComment")]
    pub line_comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(alias = "blockComment")]
    pub block_comment: Option<CharacterPair>,
}

pub type CharacterPair = Vec<String>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IndentationRules {
    #[serde(alias = "decreaseIndentPattern")]
    pub decrease_indent_pattern: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LanguageConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<CommentRule>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub brackets: Option<Vec<CharacterPair>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub word_patterns: Option<String>,

    #[serde(alias = "indentationRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indentation_rules: Option<IndentationRules>,
}

impl LanguageConfig {}

#[cfg(test)]
mod tests {
    use crate::language_config::LanguageConfig;
    use scie_infra::finder::Finder;
    use std::path::PathBuf;

    #[test]
    fn should_serialise_block_comment() {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();
        let config = root_dir
            .clone()
            .join("extensions")
            .join("java")
            .join("language-configuration.json");

        let code = Finder::read_code(&config).unwrap();

        let lang_config: LanguageConfig = serde_json::from_str(&code).unwrap();

        let block_comment = lang_config.comments.unwrap().block_comment.unwrap();
        assert_eq!("/*", block_comment[0]);
        assert_eq!("*/", block_comment[1]);
    }
}
