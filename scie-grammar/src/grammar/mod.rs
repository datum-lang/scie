pub mod grammar;
pub mod grammar_reader;
pub mod line_tokens;

pub mod local_stack_element;
pub mod scope_list_element;
pub mod scope_metadata;
pub mod stack_element;

pub use self::grammar::Grammar;
pub use self::scope_list_element::ScopeListElement;
pub use self::scope_metadata::ScopeMetadata;
pub use self::stack_element::StackElement;
use scie_scanner::scanner::scie_scanner::IOnigCaptureIndex;

#[derive(Debug, Clone, Serialize)]
pub struct MatchRuleResult {
    capture_indices: Vec<IOnigCaptureIndex>,
    matched_rule_id: i32,
}
