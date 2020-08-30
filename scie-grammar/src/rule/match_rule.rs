use crate::grammar::Grammar;
use crate::inter::ILocation;
use crate::rule::{AbstractRule, Rule};
use crate::rule::{RegExpSource, RegExpSourceList};

#[derive(Clone, Debug, Serialize)]
pub struct MatchRule {
    pub rule: Rule,
    pub _match: RegExpSource,
    pub captures: Vec<Box<dyn AbstractRule>>,
}

impl MatchRule {
    pub fn new(
        location: Option<ILocation>,
        id: i32,
        name: Option<String>,
        _match: String,
        captures: Vec<Box<dyn AbstractRule>>,
    ) -> Self {
        MatchRule {
            rule: Rule {
                _type: String::from("MatchRule"),
                _location: location,
                id,
                _name: name,
                _content_name: None,
            },
            _match: RegExpSource::new(_match, id),
            captures,
        }
    }
}

impl AbstractRule for MatchRule {
    fn id(&self) -> i32 {
        self.rule.id
    }
    fn type_of(&self) -> String {
        String::from(self.rule.clone()._type)
    }
    fn collect_patterns_recursive(
        &mut self,
        grammar: &mut Grammar,
        out: &mut RegExpSourceList,
        is_first: bool,
    ) {
        out.push(self._match.clone());
    }
}
