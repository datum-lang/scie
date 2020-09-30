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
    use std::fs::File;
    use std::io::Read;
    use std::path::PathBuf;

    fn read_code(lang_test_dir: &PathBuf) -> String {
        let mut file = File::open(lang_test_dir).unwrap();
        let mut code = String::new();
        file.read_to_string(&mut code).unwrap();
        code
    }

    #[test]
    fn should_build_oniguruma_makefile() {
        let code = "";
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();
        let lang_test_dir = root_dir
            .join("fixtures")
            .join("samples")
            .join("makefile")
            .join("oniguruma")
            .join("Makefile");
        let code = read_code(&lang_test_dir);

        let mut grammar = to_grammar_with_code("test-cases/first-mate/fixtures/makefile.json", &*code);
        assert_eq!(grammar.rule_id2desc.len(), 22);
    }

    #[test]
    fn should_build_from_simple_json() {
        let code = "";
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();
        let lang_test_dir = root_dir
            .join("scie-grammar")
            .join("test-cases")
            .join("e2e")
            .join("json")
            .join("simple-json.json");
        let code = read_code(&lang_test_dir);

        let mut grammar = to_grammar_with_code("test-cases/first-mate/fixtures/json.json", &*code);
        assert_eq!(grammar.rule_id2desc.len(), 22);
    }
}
