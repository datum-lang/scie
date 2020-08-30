use crate::rule::{AbstractRule, IRuleRegistry};
use crate::grammar::{ScopeListElement, Grammar};

#[derive(Debug, Clone, PartialEq, Eq)]
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
    pub fn null() -> Self {
        Self {
            parent: None,
            depth: 0,
            rule_id: 0,
            enter_pos: 0,
            anchor_pos: 0,
            begin_rule_captured_eol: false,
            end_rule: None,
            name_scopes_list: Default::default(),
            content_name_scopes_list: Default::default(),
        }
    }

    pub fn pop(&self) -> Option<Box<StackElement>> {
        self.clone().parent
    }
    pub fn get_rule(&self, grammar: &mut Grammar) -> Box<dyn AbstractRule> {
        grammar.get_rule(self.rule_id)
    }
    pub fn new(
        parent: Option<Box<StackElement>>,
        rule_id: i32,
        enter_pos: i32,
        anchor_pos: i32,
        begin_rule_captured_eol: bool,
        end_rule: Option<String>,
        name_scopes_list: ScopeListElement,
        content_name_scopes_list: ScopeListElement,
    ) -> Self {
        StackElement {
            parent,
            // todo: this.depth = (this.parent ? this.parent.depth + 1 : 1);
            depth: 1,
            rule_id,
            enter_pos,
            anchor_pos,
            begin_rule_captured_eol,
            end_rule,
            name_scopes_list,
            content_name_scopes_list,
        }
    }
}
