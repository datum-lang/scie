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
        _initial_scope_name: String,
        _initial_language: i32,
        _configuration: IGrammarConfiguration,
    ) {

    }
    // todo: modify logic to here for _collectDependenciesForDep
    pub fn load_grammar(
        &self,
        _initial_scope_name: String,
        _initial_language: i32,
        _configuration: IGrammarConfiguration,
    ) {
    }
}
