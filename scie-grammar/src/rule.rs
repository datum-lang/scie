use crate::inter::{IRawRepository, IRawGrammar};

pub struct RuleFactory {}

impl RuleFactory {
    pub fn get_compiled_rule_id(repository: IRawRepository) {

    }

    pub fn create_capture_rule() {}
}

// todo: trait with types
// https://users.rust-lang.org/t/impl-trait-with-generic-function-for-generic-struct/27083/2
pub trait IRuleRegistry {
    fn get_rule(&self, pattern_id: number) -> Rule;
    // fn register_rule(&self);
}

pub trait IGrammarRegistry {
    fn get_external_grammar(&self, scope_name: String, repository: IRawRepository) -> Option<IRawGrammar>;
}