use std::collections::HashMap;

pub struct IEmbeddedLanguagesMap {
    map: HashMap<String, i32>,
}

pub enum StandardTokenType {
    Other,
    Comment,
    String,
    RegEx,
}

pub struct ITokenTypeMap {
    map: HashMap<String, StandardTokenType>,
}

pub struct IGrammarConfiguration {
    pub embedded_languages: IEmbeddedLanguagesMap,
    pub token_types: ITokenTypeMap,
}

pub struct GrammarRegistry {}

impl GrammarRegistry {
    pub fn load_grammar_with_configuration(
        &self,
        initial_scope_name: String,
        initial_language: i32,
        configuration: IGrammarConfiguration,
    ) {
    }
}
