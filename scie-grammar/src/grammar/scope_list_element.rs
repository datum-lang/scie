use std::collections::HashMap;
use std::collections::hash_map::RandomState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopeListElement {
    pub parent: Option<Box<ScopeListElement>>,
    pub scope: String,
    pub metadata: i32,
}

impl ScopeListElement {
    pub fn new(parent: Option<Box<ScopeListElement>>, scope: String, metadata: i32) -> Self {
        ScopeListElement {
            parent,
            scope,
            metadata,
        }
    }

    pub fn generate_scopes(&self) -> HashMap<i32, String> {
        let mut result = HashMap::new();
        let mut resultLen = 0;

        let mut scope_list = self.clone();
        let mut is_scope_list_none = false;
        while !is_scope_list_none  {
            result.insert(resultLen, scope_list.scope.clone());
            match scope_list.parent {
                None => {
                    is_scope_list_none = true
                },
                Some(scope_value) => {
                    scope_list = *scope_value.clone();
                },
            }
        }

        return result.clone();
    }
}

impl Default for ScopeListElement {
    fn default() -> Self {
        ScopeListElement {
            parent: None,
            scope: "".to_string(),
            metadata: 0,
        }
    }
}
