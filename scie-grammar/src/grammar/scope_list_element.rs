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

    pub fn generate_scopes(&self) -> Vec<String> {
        let result: Vec<String> = vec![];
        let result_len = 0;


        return result
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
