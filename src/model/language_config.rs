pub struct CommentRule {
    #[serde(alias = "lineComment")]
    pub line_comment: String,
    #[serde(alias = "blockComment")]
    pub block_comment: CharacterPair,
}

type CharacterPair = Vec<String>;

pub struct LanguageConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: CommentRule,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brackets: Vec<CharacterPair>,
}

impl LanguageConfig {

}