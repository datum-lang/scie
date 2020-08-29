use crate::rule::rule_factory::ICompilePatternsResult;
use crate::rule::{Rule, AbstractRule};
use crate::inter::ILocation;

#[derive(Clone, Debug, Serialize)]
pub struct IncludeOnlyRule {
    #[serde(flatten)]
    pub rule: Rule,
    pub patterns: Vec<i32>,
    pub has_missing_patterns: bool,
}

impl IncludeOnlyRule {
    pub fn new(
        location: Option<ILocation>,
        id: i32,
        name: Option<String>,
        content_name: Option<String>,
        captures: ICompilePatternsResult,
    ) -> Self {
        IncludeOnlyRule {
            rule: Rule {
                _type: String::from("IncludeOnlyRule"),
                _location: location,
                id,
                _name: name,
                _content_name: content_name,
            },
            patterns: captures.patterns,
            has_missing_patterns: captures.has_missing_patterns,
        }
    }
}

impl AbstractRule for IncludeOnlyRule {
    fn id(&self) -> i32 { self.rule.id }
    fn type_of(&self) -> String {
        String::from(self.rule.clone()._type)
    }

    fn has_missing_pattern(&self) -> bool {
        self.has_missing_patterns
    }
}
