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

#[cfg(test)]
mod tests {
    use crate::grammar::grammar::to_grammar_with_code;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn should_parse_in_markdown_file() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("test-cases/e2e/makefile/Makefile");

        let code = fs::read_to_string(path).unwrap();
        let mut grammar =
            to_grammar_with_code("test-cases/first-mate/fixtures/makefile.json", &*code);
    }

}