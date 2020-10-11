#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommentRule {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(alias = "lineComment")]
    pub line_comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(alias = "blockComment")]
    pub block_comment: Option<CharacterPair>,
}

type CharacterPair = Vec<String>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IndentationRules {
    #[serde(alias = "decreaseIndentPattern")]
    pub decrease_indent_pattern: Option<String>
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

impl LanguageConfig {

}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::language_config::LanguageConfig;
    use std::fs::File;
    use std::io::Read;

    pub fn read_code(lang_test_dir: &PathBuf) -> String {
        let mut file = File::open(lang_test_dir).unwrap();
        let mut code = String::new();
        file.read_to_string(&mut code).unwrap();
        code
    }

    #[test]
    fn should_serialise_block_comment() {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf();
        let config = root_dir.clone()
            .join("extensions").join("java").join("language-configuration.json");

        let code = read_code(&config);

        let lang_config: LanguageConfig = serde_json::from_str(&code).unwrap();

        let block_comment = lang_config.comments.unwrap().block_comment.unwrap();
        assert_eq!("/*", block_comment[0]);
        assert_eq!("*/", block_comment[1]);
    }

    #[test]
    fn should_serialise_word_pattern() {
        let code = "{}";
        let lang_config: LanguageConfig = serde_json::from_str(&code).unwrap();
        println!("{:?}", lang_config);
    }
}