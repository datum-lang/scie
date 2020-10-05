use crate::grammar::Grammar;
use crate::inter::ILocation;
use crate::rule::abstract_rule::RuleEnum;
use crate::rule::rule_factory::ICompilePatternsResult;
use crate::rule::{AbstractRule, CompiledRule, IRuleRegistry, RegExpSourceList, Rule};

#[derive(Clone, Debug, Serialize)]
pub struct IncludeOnlyRule {
    #[serde(flatten)]
    pub rule: Rule,
    pub patterns: Vec<i32>,
    pub has_missing_patterns: bool,
    pub _cached_compiled_patterns: Option<RegExpSourceList>,
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
            _cached_compiled_patterns: None,
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
    fn get_rule(&self) -> Rule {
        self.rule.clone()
    }
    fn get_rule_instance(&self) -> RuleEnum {
        RuleEnum::IncludeOnlyRule(self.clone())
    }
    fn has_missing_pattern(&self) -> bool {
        self.has_missing_patterns
    }
    fn patterns_length(&self) -> i32 {
        self.patterns.clone().len() as i32
    }

    fn collect_patterns_recursive(
        &mut self,
        grammar: &mut Grammar,
        out: &mut RegExpSourceList,
        _is_first: bool,
    ) {
        for x in self.patterns.clone() {
            let mut rule = grammar.get_rule(x);
            rule.collect_patterns_recursive(grammar, out, false);
        }
    }

    fn compile(
        &mut self,
        grammar: &mut Grammar,
        _end_regex_source: Option<String>,
        allow_a: bool,
        allow_g: bool,
    ) -> CompiledRule {
        if let None = self._cached_compiled_patterns {
            let mut cached_compiled_patterns = RegExpSourceList::new();

            self.collect_patterns_recursive(grammar, &mut cached_compiled_patterns, true);
            self._cached_compiled_patterns = Some(cached_compiled_patterns.clone());
        }

        return *self._cached_compiled_patterns.as_mut().unwrap().compile(allow_a, allow_g);
    }
}
