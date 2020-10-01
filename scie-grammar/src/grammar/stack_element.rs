use crate::grammar::{Grammar, ScopeListElement};
use crate::rule::{AbstractRule, IRuleRegistry};

// todo: change to rccall https://stackoverflow.com/questions/36167160/how-do-i-express-mutually-recursive-data-structures-in-safe-rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StackElement {
    pub parent: Option<Box<StackElement>>,
    pub depth: i32,
    #[serde(rename = "RuleId")]
    pub rule_id: i32,
    pub enter_pos: i32,
    #[serde(rename = "_anchorPos")]
    pub anchor_pos: i32,
    #[serde(rename = "beginRuleCapturedEOL")]
    pub begin_rule_captured_eol: bool,
    #[serde(rename = "endRule")]
    pub end_rule: Option<String>,
    #[serde(rename = "nameScopesList")]
    pub name_scopes_list: ScopeListElement,
    #[serde(rename = "contentNameScopesList")]
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

    pub fn pop(&self) -> Option<StackElement> {
        match self.parent.clone() {
            None => None,
            Some(parents) => Some(*parents.clone()),
        }
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
        let mut depth = 1;
        if let Some(iparent) = parent.clone() {
            depth = iparent.depth + 1
        }
        let mut element = StackElement {
            parent,
            depth,
            rule_id,
            enter_pos,
            anchor_pos,
            begin_rule_captured_eol,
            end_rule,
            name_scopes_list,
            content_name_scopes_list,
        };

        element
    }

    pub fn stringify(self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn push(
        &self,
        rule_id: i32,
        enter_pos: i32,
        anchor_pos: i32,
        begin_rule_captured_eol: bool,
        end_rule: Option<String>,
        name_scopes_list: ScopeListElement,
        content_name_scopes_list: ScopeListElement,
    ) -> StackElement {
        StackElement::new(
            Some(Box::from(self.clone())),
            rule_id,
            enter_pos,
            anchor_pos,
            begin_rule_captured_eol,
            end_rule,
            name_scopes_list,
            content_name_scopes_list,
        )
    }

    pub fn set_content_name_scopes_list(
        self,
        content_name_scopes_list: ScopeListElement,
    ) -> StackElement {
        if self.content_name_scopes_list == content_name_scopes_list {
            return self;
        }

        panic!("todo: set_content_name_scopes_list");
        // return self;
    }

    pub fn reset(&mut self) {
        self.enter_pos = -1;
        self.anchor_pos = -1;

        if let Some(_parent) = self.parent.clone() {
            self.parent.as_mut().unwrap().reset();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::grammar::{ScopeListElement, StackElement};

    #[test]
    fn should_reset_parent() {
        let mut node = StackElement::null();
        node.anchor_pos = 1;
        node.enter_pos = 1;

        let element = ScopeListElement::new(None, String::from("scope"));
        let mut new_node = node.push(1, 0, 0, false, None, element.clone(), element.clone());

        new_node.reset();

        assert_eq!(new_node.enter_pos, -1);
        assert_eq!(new_node.anchor_pos, -1);

        let parent = new_node.parent.unwrap();
        assert_eq!(parent.anchor_pos, -1);
        assert_eq!(parent.enter_pos, -1);
    }
}
