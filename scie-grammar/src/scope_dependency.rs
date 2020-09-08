use std::collections::{HashSet, HashMap};
use crate::inter::IRawRule;

#[derive(Clone, Debug, Serialize)]
pub struct PartialScopeDependency {
    pub scope_name: String,
    pub include: String
}

impl PartialScopeDependency {
    pub fn to_key(&self) -> String {
        format!("{:?}#{:?}", self.scope_name, self.include)
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct FullScopeDependency {
    pub scope_name: String
}

#[derive(Clone, Debug, Serialize)]
pub enum ScopeDependency {
    FullScopeDependency(FullScopeDependency),
    PartialScopeDependency(PartialScopeDependency)
}

#[derive(Clone, Debug, Serialize)]
pub struct ScopeDependencyCollector {
    pub full: Vec<FullScopeDependency>,
    pub partial: Vec<PartialScopeDependency>,
    pub visited_rule: HashMap<String, IRawRule>,
    pub _seen_full: HashSet<String>,
    pub _seen_partial: HashSet<String>,
}

impl ScopeDependencyCollector {
    pub fn new() -> ScopeDependencyCollector {
        ScopeDependencyCollector {
            full: vec![],
            partial: vec![],
            visited_rule: Default::default(),
            _seen_full: Default::default(),
            _seen_partial: Default::default()
        }
    }

    pub fn add(&mut self, dep: ScopeDependency) {
        match dep {
            ScopeDependency::FullScopeDependency(full_dep) => {
                let scope_name = &*full_dep.scope_name.clone();
                if let None = self._seen_full.get(scope_name.clone()) {
                    self._seen_full.insert(String::from(scope_name));
                    self.full.push(full_dep);
                }
            },
            ScopeDependency::PartialScopeDependency(partial_dep) => {
                let key = &*partial_dep.to_key();
                if let None = self._seen_partial.get(key) {
                    self._seen_partial.insert(String::from(key));
                    self.partial.push(partial_dep);
                }
            },
        }
    }
}

