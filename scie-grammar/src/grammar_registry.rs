use std::collections::{HashMap, HashSet};
use crate::scope_dependency::{ScopeDependency, FullScopeDependency};
use crate::scope_dependency::ScopeDependency::Full;

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
    ) {}
    // todo: modify logic to here for _collectDependenciesForDep
    pub fn _load_grammar(&self, initial_scope_name: String, _initial_language: i32, _token_type: Option<ITokenTypeMap>) {
        let mut seen_full_scope_requests: HashSet<String> = HashSet::new();
        let seen_partial_scope_requests: HashSet<String> = HashSet::new();

        seen_full_scope_requests.insert(initial_scope_name.clone());

        let dependency = FullScopeDependency::new(initial_scope_name);
        let mut Q: Vec<ScopeDependency> = vec![ScopeDependency::Full(dependency)];

        while Q.len() > 0 {
            let q = Q.clone();
            Q = vec![];

            for x in q {
                match x {
                    Full(dep) => {
                        self._loadSingleGrammar(dep.scope_name);
                    },
                    ScopeDependency::Partial(dep) => {
                        self._loadSingleGrammar(dep.scope_name);
                    },
                }
            }
        }
    }

    pub fn _loadSingleGrammar(&self, scope_name: String) {
        // todo: add cache supportr
    }

    pub fn load_grammar(&self, initial_scope_name: String) {
        self._load_grammar(initial_scope_name, 0, None)
    }
}
