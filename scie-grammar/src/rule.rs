use crate::grammar::grammar::Grammar;
use crate::inter::{ILocation, IRawCaptures, IRawGrammar, IRawRepository, IRawRule};
use core::fmt;
use dyn_clone::{clone_trait_object, DynClone};

#[derive(Clone, Debug)]
pub struct ICompilePatternsResult {
    pub patterns: Vec<i32>,
    pub has_missing_patterns: bool
}

pub struct RuleFactory {}

impl RuleFactory {
    // todo: add more rule builder
    fn compile_captures(
        captures: Option<Box<IRawCaptures>>,
        helper: &mut Grammar,
        repository: IRawRepository,
    ) -> Vec<CaptureRule> {
        let mut r = vec![];

        if let Some(capts) = captures.clone() {
            let mut maximum_capture_id = 0;
            for (id_str, value) in capts.clone().map.capture_map {
                let id: i32 = id_str.parse().unwrap_or(0);
                if id > maximum_capture_id {
                    maximum_capture_id = id
                }
            }
            for i in 0..maximum_capture_id {
                r.push(CaptureRule::new());
            }

            let cloned_capts = captures.clone().unwrap();
            for (id_str, value) in capts.clone().map.capture_map {
                let numeric_capture_id: i32 = id_str.parse().unwrap_or(0);
                let mut retokenizeCapturedWithRuleId = 0;
                println!("{:?}", numeric_capture_id.clone().to_string());
                let options_patterns = cloned_capts
                    .map
                    .capture_map
                    .get(&*numeric_capture_id.to_string());

                let desc = captures.clone().unwrap().map.capture_map[&id_str].clone();
                if let Some(rule) = options_patterns {
                    retokenizeCapturedWithRuleId = RuleFactory::get_compiled_rule_id(desc, helper, repository.clone());
                }
                // r[numericCaptureId] = RuleFactory::create_capture_rule(helper, desc.location, desc.name, desc.content_name, retokenizeCapturedWithRuleId);
            }
        };

        r
    }

    pub fn compile_patterns(
        patterns: Option<Vec<IRawRule>>,
        helper: Box<&mut dyn IRuleFactoryHelper>,
        repository: IRawRepository
    ) -> ICompilePatternsResult {
        let mut r: Vec<i32> = vec![];

        let result = ICompilePatternsResult {
            patterns: r,
            has_missing_patterns: false
        };

        result
    }

    pub fn get_compiled_rule_id(
        mut desc: IRawRule,
        helper: &mut Grammar,
        repository: IRawRepository,
    ) -> i32 {
        if let None = desc.id {
            let id = helper.register_id();
            desc.id = Some(id.clone());

            if let Some(match_s) = desc.match_s {
                let rule_factory = RuleFactory::compile_captures(
                    desc.captures.clone(),
                    helper,
                    repository.clone(),
                );
                let rule = MatchRule::new(
                    desc.location.clone(),
                    id.clone(),
                    desc.name.clone(),
                    match_s.clone(),
                    rule_factory, );

                helper.register_rule(Box::new(rule));
                return desc.id.unwrap();
            };

            if let None = desc.begin {
                if let Some(repo) = desc.repository {
                    //todo: mergeObjects
                }
                let mut patterns = desc.patterns;
                if let None = patterns {
                    if let Some(include) = desc.include {
                        let mut raw_rule = IRawRule::new();
                        raw_rule.include = Some(include);
                        patterns = Some(vec![raw_rule.clone()])
                    }
                }

                let rule_factory = RuleFactory::compile_patterns(
                    patterns.clone(),
                    Box::new(helper),
                    repository.clone(),
                );
                let rule = IncludeOnlyRule::new(
                    desc.location.clone(),
                    desc.id.unwrap().clone(),
                    desc.name.clone(),
                    desc.content_name.clone(),
                    rule_factory,
                );
                helper.register_rule(Box::new(rule));
                return desc.id.unwrap();
            }

            
        }

        desc.id.unwrap()
    }

    pub fn create_capture_rule() {}
}

#[derive(Clone, Debug)]
pub struct Rule {
    pub location: Option<ILocation>,
    pub id: i32,
    pub name: Option<String>,
    pub content_name: Option<String>,
}

impl Rule {
    pub fn new(
        location: ILocation,
        id: i32,
        name: Option<String>,
        content_name: Option<String>,
    ) -> Self {
        Rule {
            location: Some(location),
            id,
            name,
            content_name,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RegExpSource {}

pub trait AbstractRule: DynClone {}

impl fmt::Debug for dyn AbstractRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AbstractRule")
    }
}

clone_trait_object!(AbstractRule);

#[derive(Clone, Debug)]
pub struct IncludeOnlyRule {
    pub rule: Rule,
    pub _match: RegExpSource,
    pub captures: ICompilePatternsResult,
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
                location,
                id,
                name,
                content_name: None,
            },
            _match: RegExpSource {},
            captures,
        }
    }
}

impl AbstractRule for IncludeOnlyRule {}

#[derive(Clone, Debug)]
pub struct BeginWhileRule {
    pub rule: Rule,
}

impl AbstractRule for BeginWhileRule {}

#[derive(Clone, Debug)]
pub struct MatchRule {
    pub rule: Rule,
    pub _match: RegExpSource,
    pub captures: Vec<CaptureRule>,
}

impl MatchRule {
    pub fn new(
        location: Option<ILocation>,
        id: i32,
        name: Option<String>,
        match_s: String,
        captures: Vec<CaptureRule>,
    ) -> Self {
        MatchRule {
            rule: Rule {
                location,
                id,
                name,
                content_name: None,
            },
            _match: RegExpSource {},
            captures,
        }
    }
}

impl AbstractRule for MatchRule {}

#[derive(Clone, Debug)]
pub struct BeginEndRule {
    pub rule: Rule,
}

impl AbstractRule for BeginEndRule {}

#[derive(Clone, Debug)]
pub struct CaptureRule {
    pub rule: Rule,
}

impl CaptureRule {
    pub fn new() -> Self {
        CaptureRule {
            rule: Rule {
                location: None,
                id: 0,
                name: None,
                content_name: None,
            },
        }
    }
}

impl AbstractRule for CaptureRule {}

// todo: trait with types
// https://users.rust-lang.org/t/impl-trait-with-generic-function-for-generic-struct/27083/2
pub trait IRuleRegistry {
    // type Output;
    // fn method(&self) -> Self::Output;

    fn register_id(&mut self) -> i32;
    fn get_rule(&self, pattern_id: i32) -> Rule;
    fn register_rule(&mut self, result: Box<dyn AbstractRule>) -> Box<dyn AbstractRule>;
}

pub trait IGrammarRegistry {
    fn get_external_grammar(
        &self,
        scope_name: String,
        repository: IRawRepository,
    ) -> Option<IRawGrammar>;
}

pub trait IRuleFactoryHelper: IGrammarRegistry + IRuleRegistry {}
