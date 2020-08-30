use crate::inter::ILocation;
use crate::rule::rule_factory::ICompilePatternsResult;
use crate::rule::{AbstractRule, Rule, RegExpSourceList, IRuleRegistry};
use crate::grammar::Grammar;

#[derive(Clone, Debug, Serialize)]
pub struct IncludeOnlyRule {
    #[serde(flatten)]
    pub rule: Rule,
    pub patterns: Vec<i32>,
    pub has_missing_patterns: bool,
    pub _cached_compiled_patterns: Option<RegExpSourceList>
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
            _cached_compiled_patterns: None
        }
    }
}

impl AbstractRule for IncludeOnlyRule {
    fn id(&self) -> i32 {
        self.rule.id
    }
    fn type_of(&self) -> String {
        String::from(self.rule.clone()._type)
    }
    fn has_missing_pattern(&self) -> bool {
        self.has_missing_patterns
    }

    fn collect_patterns_recursive(&mut self, grammar: &mut Grammar, out: Option<RegExpSourceList>, is_first: bool) {
        for x in self.patterns.clone() {
            let mut rule = grammar.get_rule(x);
            rule.collect_patterns_recursive(grammar, out.clone(), is_first);
        }
    }


    fn compile(&mut self, grammar: &mut Grammar, end_regex_source: Option<String>, allow_a: bool, allow_g: bool) {
        if let None = self._cached_compiled_patterns {
            self._cached_compiled_patterns = Some(RegExpSourceList::new());
            self.collect_patterns_recursive(grammar, self._cached_compiled_patterns.clone(), true);
        }

        return self._cached_compiled_patterns.as_ref().unwrap().compile(grammar, allow_a, allow_g);
    }
}
