use crate::rule::{AbstractRule, Rule};
use crate::rule::reg_exp_source::{RegExpSource, RegExpSourceList};
use crate::rule::rule_factory::ICompilePatternsResult;
use crate::inter::ILocation;

#[derive(Clone, Debug, Serialize)]
pub struct BeginWhileRule {
    pub rule: Rule,

    pub _begin: RegExpSource,
    pub begin_captures: Vec<Box<dyn AbstractRule>>,

    pub _while: Option<String>,
    pub while_captures: Vec<Box<dyn AbstractRule>>,

    pub apply_end_pattern_last: bool,

    pub patterns: ICompilePatternsResult,
    pub has_missing_patterns: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached_compiled_patterns: Option<RegExpSourceList>,
}

impl BeginWhileRule {
    pub fn new(
        location: Option<ILocation>,
        id: i32,
        name: Option<String>,
        content_name: Option<String>,
        _begin: Option<String>,
        begin_captures: Vec<Box<dyn AbstractRule>>,
        _while: Option<String>,
        while_captures: Vec<Box<dyn AbstractRule>>,
        patterns: ICompilePatternsResult,
    ) -> BeginWhileRule {
        BeginWhileRule {
            rule: Rule {
                _type: String::from("BeginWhileRule"),
                _location: location,
                id,
                _name: name,
                _content_name: content_name,
            },
            _begin: RegExpSource::new(_begin.unwrap().clone(), id.clone()),
            begin_captures,
            _while,
            while_captures,
            apply_end_pattern_last: false,
            has_missing_patterns: patterns.clone().has_missing_patterns,
            patterns,
            cached_compiled_patterns: None,
        }
    }
}

impl AbstractRule for BeginWhileRule {
    fn id(&self) -> i32 { self.rule.id }
    fn type_of(&self) -> String {
        String::from(self.rule.clone()._type)
    }
    fn has_missing_pattern(&self) -> bool {
        self.has_missing_patterns
    }
}
