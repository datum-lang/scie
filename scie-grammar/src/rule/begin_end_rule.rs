use crate::inter::ILocation;
use crate::rule::rule_factory::ICompilePatternsResult;
use crate::rule::{AbstractRule, Rule, IRuleRegistry, CompiledRule};
use crate::rule::{RegExpSource, RegExpSourceList};
use crate::grammar::Grammar;

#[derive(Clone, Debug, Serialize)]
pub struct BeginEndRule {
    #[serde(flatten)]
    pub rule: Rule,
    pub _begin: RegExpSource,
    pub begin_captures: Vec<Box<dyn AbstractRule>>,
    pub _end: RegExpSource,
    // pub endHasBackReferences: Option<bool>,
    pub end_captures: Vec<Box<dyn AbstractRule>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_end_pattern_last: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _cached_compiled_patterns: Option<RegExpSourceList>,
    pub patterns: Vec<i32>,
    pub has_missing_patterns: bool,
}

impl BeginEndRule {
    pub fn new(
        location: Option<ILocation>,
        id: i32,
        name: Option<String>,
        content_name: Option<String>,
        begin: String,
        begin_captures: Vec<Box<dyn AbstractRule>>,
        _end: String,
        end_captures: Vec<Box<dyn AbstractRule>>,
        apply_end_pattern_last: Option<bool>,
        patterns: ICompilePatternsResult,
    ) -> BeginEndRule {
        BeginEndRule {
            rule: Rule {
                _type: String::from("BeginEndRule"),
                _location: location,
                id,
                _name: name,
                _content_name: content_name,
            },
            _begin: RegExpSource::new(begin.clone(), id.clone()),
            begin_captures,
            _end: RegExpSource::new(_end.clone(), id.clone()),
            end_captures,
            apply_end_pattern_last,
            has_missing_patterns: patterns.clone().has_missing_patterns,
            patterns: patterns.patterns,
            _cached_compiled_patterns: None,
        }
    }
}

impl AbstractRule for BeginEndRule {
    fn id(&self) -> i32 {
        self.rule.id
    }
    fn type_of(&self) -> String {
        String::from(self.rule.clone()._type)
    }
    fn has_missing_pattern(&self) -> bool {
        self.has_missing_patterns
    }

    fn collect_patterns_recursive(&mut self, grammar: &mut Grammar, out: &mut RegExpSourceList, is_first: bool) {
        if is_first {
            for x in self.patterns.clone() {
                let mut rule = grammar.get_rule(x);
                rule.collect_patterns_recursive(grammar, out, is_first);
            }
        } else {
            out.push(self._begin.clone());
        }
    }

    fn compile(&mut self, grammar: &mut Grammar, end_regex_source: Option<String>, allow_a: bool, allow_g: bool) -> CompiledRule {
        if let None = self._cached_compiled_patterns {
            // todo: figured cached issues
            let mut cached_compiled_patterns = RegExpSourceList::new();
            self.collect_patterns_recursive(grammar, &mut cached_compiled_patterns, true);

            if let Some(apply_end) = self.apply_end_pattern_last {
                if apply_end {
                    cached_compiled_patterns.push(self._end.clone());
                } else {
                    cached_compiled_patterns.unshift(self._end.clone());
                }
            } else {
                cached_compiled_patterns.unshift(self._end.clone());
            }

            self._cached_compiled_patterns = Some(cached_compiled_patterns);
        }

        // todo: support for hasBackReferences
        self._cached_compiled_patterns.as_ref().unwrap().compile(grammar, allow_a, allow_g)
    }
}
