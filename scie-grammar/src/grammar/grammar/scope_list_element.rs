pub struct ScopeListElement {
    pub parent: Option<Box<ScopeListElement>>,
    pub scope: String,
    pub metadata: i32
}