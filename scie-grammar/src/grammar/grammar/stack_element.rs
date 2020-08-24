use crate::grammar::grammar::scope_list_element::ScopeListElement;

pub struct StackElement {
    pub parent: Option<Box<StackElement>>,
    pub depth: i32,
    pub rule_id: i32,
    pub enter_pos: i32,
    pub anchor_pos: i32,
    pub begin_rule_captured_eol: bool,
    pub end_rule: Option<String>,
    pub name_scopes_list: ScopeListElement,
    pub content_name_scopes_list: ScopeListElement,
}

impl StackElement {
    pub fn null() {}
}
