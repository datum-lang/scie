use crate::inter::{IRawRepository, IRawGrammar, ILocation};

pub struct RuleFactory {}

impl RuleFactory {
    pub fn get_compiled_rule_id(repository: IRawRepository) {}

    pub fn create_capture_rule() {}
}


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

pub trait AbstractRule {

}

pub struct IncludeOnlyRule {
    pub rule: Rule
}

impl AbstractRule for IncludeOnlyRule {}

pub struct BeginWhileRule {
    pub rule: Rule
}

impl AbstractRule for BeginWhileRule {}

pub struct MatchRule {
    pub rule: Rule
}

impl AbstractRule for MatchRule {}

pub struct BeginEndRule {
    pub rule: Rule
}

impl AbstractRule for BeginEndRule {}

pub struct CaptureRule {
    pub rule: Rule
}

impl AbstractRule for CaptureRule {}


// todo: trait with types
// https://users.rust-lang.org/t/impl-trait-with-generic-function-for-generic-struct/27083/2
pub trait IRuleRegistry {
    fn get_rule(&self, pattern_id: i32) -> Rule;
    // fn register_rule(&self);
}

pub trait IGrammarRegistry {
    fn get_external_grammar(&self, scope_name: String, repository: IRawRepository) -> Option<IRawGrammar>;
}