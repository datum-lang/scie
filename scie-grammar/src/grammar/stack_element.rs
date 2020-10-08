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
        if self.parent.is_some() {
            return Some(*self.parent.to_owned().unwrap());
        }
        return None;
    }
    //
    // pub fn get_rule<'a, 'b: 'a>(&self, grammar: &'b mut Grammar) -> &'b Option<&mut Box<dyn AbstractRule>> {
    //     grammar.get_rule(self.rule_id)
    // }

    pub fn update_rule(&self, grammar: &mut Grammar, rule: Box<dyn AbstractRule>) -> i32 {
        grammar.register_rule(rule)
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
        if parent.is_some() {
            depth = parent.as_ref().unwrap().depth.clone() + 1
        }

        StackElement {
            parent,
            depth,
            rule_id,
            enter_pos,
            anchor_pos,
            begin_rule_captured_eol,
            end_rule,
            name_scopes_list,
            content_name_scopes_list,
        }
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
            Some(Box::from(self.to_owned())),
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
        &mut self,
        content_name_scopes_list: ScopeListElement,
    ) -> StackElement {
        if self.content_name_scopes_list == content_name_scopes_list {
            return self.to_owned();
        }

        self.parent.as_mut().unwrap().push(
            self.rule_id,
            self.enter_pos,
            self.anchor_pos,
            self.begin_rule_captured_eol,
            self.end_rule.clone(),
            self.name_scopes_list.clone(),
            content_name_scopes_list,
        )
    }

    pub fn reset(&mut self) {
        self.enter_pos = -1;
        self.anchor_pos = -1;

        if self.parent.is_some() {
            self.parent.as_mut().unwrap().reset();
        }
    }

    pub fn set_end_rule(&self, end_rule: String) -> StackElement {
        if self.end_rule.is_some() {
            if *self.end_rule.as_ref().unwrap() == end_rule {
                return self.clone();
            }
        }

        return StackElement::new(
            self.parent.clone(),
            self.rule_id,
            self.enter_pos,
            self.anchor_pos,
            self.begin_rule_captured_eol,
            Option::from(end_rule.clone()),
            self.name_scopes_list.clone(),
            self.content_name_scopes_list.clone(),
        );
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
