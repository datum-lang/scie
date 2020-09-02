use crate::grammar::Grammar;
use crate::inter::ILocation;
use crate::rule::rule_factory::ICompilePatternsResult;
use crate::rule::{AbstractRule, CompiledRule, IRuleRegistry, Rule};
use crate::rule::{RegExpSource, RegExpSourceList};

#[derive(Clone, Debug, Serialize)]
pub struct BeginWhileRule {
    pub rule: Rule,

    pub _begin: RegExpSource,
    pub begin_captures: Vec<Box<dyn AbstractRule>>,

    pub _while: Option<String>,
    pub while_captures: Vec<Box<dyn AbstractRule>>,

    pub apply_end_pattern_last: bool,

    pub patterns: Vec<i32>,
    pub has_missing_patterns: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _cached_compiled_patterns: Option<RegExpSourceList>,
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
            patterns: patterns.patterns,
            _cached_compiled_patterns: None,
        }
    }
}

impl AbstractRule for BeginWhileRule {
    fn id(&self) -> i32 {
        self.rule.id
    }
    fn get_rule(&self) -> Rule {
        self.rule.clone()
    }
    fn type_of(&self) -> String {
        String::from(self.rule.clone()._type)
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
        is_first: bool,
    ) {
        if is_first {
            for x in self.patterns.clone() {
                let mut rule = grammar.get_rule(x);
                rule.collect_patterns_recursive(grammar, out, false);
            }
        } else {
            out.push(Box::new(self._begin.clone()));
        }
    }

    fn compile(
        &mut self,
        grammar: &mut Grammar,
        end_regex_source: Option<String>,
        allow_a: bool,
        allow_g: bool,
    ) -> CompiledRule {
        let mut cached_compiled_patterns = RegExpSourceList::new();

        if let None = self._cached_compiled_patterns {
            self.collect_patterns_recursive(grammar, &mut cached_compiled_patterns, true);
            self._cached_compiled_patterns = Some(cached_compiled_patterns.clone());
        } else {
            cached_compiled_patterns = self._cached_compiled_patterns.as_ref().unwrap().clone();
        }

        return cached_compiled_patterns
            .compile(grammar, allow_a, allow_g)
            .clone();
    }
}
