use crate::inter::IRawRule;
use std::collections::HashSet;

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
    // pub visited_rule: HashSet<IRawRule>,
    pub _seen_full: HashSet<String>,
    pub _seen_partial: HashSet<String>,
}

impl ScopeDependencyCollector {
    pub fn new() -> ScopeDependencyCollector {
        ScopeDependencyCollector {
            full: vec![],
            partial: vec![],
            // visited_rule: Default::default(),
            _seen_full: Default::default(),
            _seen_partial: Default::default()
        }
    }

    pub fn add(&mut self, dep: ScopeDependency) {
        match dep {
            ScopeDependency::FullScopeDependency(full_dep) => {
                // self._seen_full.get_or_insert(full_dep.scope_name);
                // self._seen_full.get(&*full_dep.scope_name.clone());
            },
            ScopeDependency::PartialScopeDependency(_) => {

            },
        }
    }
}

