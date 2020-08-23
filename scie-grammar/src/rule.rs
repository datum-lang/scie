use crate::inter::{IRawRepository, IRawGrammar, ILocation, IRawRule};
use dyn_clone::{clone_trait_object, DynClone};
use std::borrow::Borrow;
use crate::grammar::grammar::Grammar;

pub struct RuleFactory {}

impl RuleFactory {
    pub fn get_compiled_rule_id(mut desc: IRawRule, helper: &mut Grammar, repository: IRawRepository) -> i32 {
        match desc.id {
            None => {
                helper.register_rule(|id| {
                    desc.id = Some(id);
                    let rule = BeginEndRule {
                        rule: Rule {
                            location: ILocation::new(),
                            id: id,
                            name: None,
                            content_name: None
                        }
                    };

                    Box::from(rule)
                });
            },
            Some(_) => {},
        }

        desc.id.unwrap()
    }

    pub fn create_capture_rule() {}
}


#[derive(Clone, Debug)]
pub struct Rule {
    pub location: ILocation,
    pub id: i32,
    pub name: Option<String>,
    pub content_name: Option<String>,
}

impl Rule {
    pub fn new(location: ILocation, id: i32, name: Option<String>, content_name: Option<String>) -> Self {
        Rule { location, id, name, content_name }
    }
}

pub trait AbstractRule: DynClone {}

clone_trait_object!(AbstractRule);

#[derive(Clone, Debug)]
pub struct IncludeOnlyRule {
    pub rule: Rule
}

impl AbstractRule for IncludeOnlyRule {}

#[derive(Clone, Debug)]
pub struct BeginWhileRule {
    pub rule: Rule
}

impl AbstractRule for BeginWhileRule {}

#[derive(Clone, Debug)]
pub struct MatchRule {
    pub rule: Rule
}

impl AbstractRule for MatchRule {}

#[derive(Clone, Debug)]
pub struct BeginEndRule {
    pub rule: Rule
}

impl AbstractRule for BeginEndRule {}

#[derive(Clone, Debug)]
pub struct CaptureRule {
    pub rule: Rule
}

impl AbstractRule for CaptureRule {}


// todo: trait with types
// https://users.rust-lang.org/t/impl-trait-with-generic-function-for-generic-struct/27083/2
pub trait IRuleRegistry {
    // type Output;
    // fn method(&self) -> Self::Output;

    fn get_rule(&self, pattern_id: i32) -> Rule;
    fn register_rule(&mut self, c: fn(id: i32) -> Box<dyn AbstractRule>) -> Box<dyn AbstractRule>;
}

pub trait IGrammarRegistry {
    fn get_external_grammar(&self, scope_name: String, repository: IRawRepository) -> Option<IRawGrammar>;
}

pub trait IRuleFactoryHelper: IGrammarRegistry + IRuleRegistry {}
