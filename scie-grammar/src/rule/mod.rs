pub mod rule_factory;

pub mod abstract_rule;
pub mod begin_end_rule;
pub mod begin_while_rule;
pub mod capture_rule;
pub mod compiled_rule;
pub mod empty_rule;
pub mod include_only_rule;
pub mod match_rule;
pub mod reg_exp_source;
pub mod rule;

pub use self::abstract_rule::AbstractRule;
pub use self::begin_end_rule::BeginEndRule;
pub use self::begin_while_rule::BeginWhileRule;
pub use self::capture_rule::CaptureRule;
pub use self::compiled_rule::CompiledRule;
pub use self::empty_rule::EmptyRule;
pub use self::include_only_rule::IncludeOnlyRule;
pub use self::match_rule::MatchRule;
pub use self::reg_exp_source::RegExpSource;
pub use self::reg_exp_source::RegExpSourceList;
pub use self::rule::Rule;

use crate::inter::{IRawGrammar, IRawRepository};
use std::cell::RefCell;
use std::rc::Rc;

// todo: trait with types
// https://users.rust-lang.org/t/impl-trait-with-generic-function-for-generic-struct/27083/2
pub trait IRuleRegistry {
    fn register_id(&mut self) -> i32;
    fn get_rule(&mut self, pattern_id: i32) -> Rc<RefCell<dyn AbstractRule>>;
    fn register_rule(&mut self, result: Rc<RefCell<dyn AbstractRule>>) -> i32;
}

pub trait IGrammarRegistry {
    fn get_external_grammar(
        &self,
        scope_name: String,
        repository: IRawRepository,
    ) -> Option<IRawGrammar>;
}

pub trait IRuleFactoryHelper: IGrammarRegistry + IRuleRegistry {}
