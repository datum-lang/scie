use crate::grammar::ScopeListElement;

#[derive(Debug, Clone)]
pub struct LocalStackElement {
    pub scopes: ScopeListElement,
    pub end_pos: i32,
}

impl LocalStackElement {
    pub fn new(scopes: ScopeListElement, end_pos: i32) -> Self {
        LocalStackElement { scopes, end_pos }
    }
}