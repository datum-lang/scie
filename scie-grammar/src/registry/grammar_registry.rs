use std::collections::{HashMap, HashSet};
use crate::registry::scope_dependency::{ScopeDependency, FullScopeDependency, ScopeDependencyCollector};
use crate::registry::scope_dependency::ScopeDependency::Full;

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
    // todo: modify logic to here for _collect_dependencies_for_dep
    pub fn _load_grammar(&self, initial_scope_name: String, _initial_language: i32, _token_type: Option<ITokenTypeMap>) {
        let mut seen_full_scope_requests: HashSet<String> = HashSet::new();
        let _seen_partial_scope_requests: HashSet<String> = HashSet::new();

        seen_full_scope_requests.insert(initial_scope_name.clone());

        let dependency = FullScopeDependency::new(initial_scope_name.clone());
        let mut queue: Vec<ScopeDependency> = vec![ScopeDependency::Full(dependency)];

        while queue.len() > 0 {
            let q = queue.clone();
            queue = vec![];

            for x in q.clone() {
                match x {
                    Full(dep) => {
                        self._load_single_grammar(dep.scope_name);
                    }
                    ScopeDependency::Partial(dep) => {
                        self._load_single_grammar(dep.scope_name);
                    }
                }
            }

            let mut deps = ScopeDependencyCollector::new();
            for dep in q.clone() {
                self._collect_dependencies_for_dep(initial_scope_name.clone(), &mut deps, dep);
            }
        }
    }

    pub fn _collect_dependencies_for_dep(&self, _scope_name: String, _deps: &mut ScopeDependencyCollector, _dep: ScopeDependency) {
        
    }
    pub fn _load_single_grammar(&self, _scope_name: String) {
        // todo: add cache support
        // todo: add load single gammar
    }

    pub fn load_grammar(&self, initial_scope_name: String) {
        self._load_grammar(initial_scope_name, 0, None)
    }
}
