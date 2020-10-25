use scie_scanner::scanner::scie_scanner::IOnigCaptureIndex;

pub use self::grammar::Grammar;
pub use self::scope_list_element::ScopeListElement;
pub use self::scope_metadata::ScopeMetadata;
pub use self::stack_element::StackElement;

pub mod grammar;
pub mod grammar_reader;
pub mod line_tokens;

pub mod local_stack_element;
pub mod rule_container;
pub mod scope_list_element;
pub mod scope_metadata;
pub mod stack_element;

#[derive(Debug, Clone, Serialize)]
pub struct MatchRuleResult {
    capture_indices: Vec<IOnigCaptureIndex>,
    matched_rule_id: i32,
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
    use std::path::PathBuf;

    use crate::grammar::Grammar;

    fn read_code(lang_test_dir: &PathBuf) -> String {
        let mut file = File::open(lang_test_dir).unwrap();
        let mut code = String::new();
        file.read_to_string(&mut code).unwrap();
        code
    }

    #[test]
    fn should_build_oniguruma_makefile() {
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

        let grammar = Grammar::from_code("extensions/make/syntaxes/make.tmLanguage.json", &*code);
        assert_eq!(grammar.rule_id2desc.len(), 104);
    }

    #[test]
    fn should_build_from_simple_json() {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();
        let lang_test_dir = root_dir
            .join("fixtures")
            .join("test-cases")
            .join("e2e")
            .join("json")
            .join("simple-json.json");
        let code = read_code(&lang_test_dir);

        let grammar = Grammar::from_code("extensions/json/syntaxes/JSON.tmLanguage.json", &*code);
        assert_eq!(grammar.rule_id2desc.len(), 35);
    }

    #[test]
    fn should_build_from_simple_javascript() {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();
        let lang_test_dir = root_dir
            .join("fixtures")
            .join("test-cases")
            .join("e2e")
            .join("javascript")
            .join("pairs.js");
        let code = read_code(&lang_test_dir);

        let grammar = Grammar::from_code(
            "extensions/javascript/syntaxes/JavaScript.tmLanguage.json",
            &*code,
        );
        assert_eq!(grammar.rule_id2desc.len(), 997);
    }

    #[test]
    fn should_build_from_simple_csharp() {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();
        let lang_test_dir = root_dir
            .join("fixtures")
            .join("test-cases")
            .join("e2e")
            .join("csharp")
            .join("test.cs");
        let code = read_code(&lang_test_dir);

        let grammar =
            Grammar::from_code("extensions/csharp/syntaxes/csharp.tmLanguage.json", &*code);
        assert_eq!(grammar.rule_id2desc.len(), 690);
    }

    #[test]
    fn should_build_from_simple_rust() {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();
        let lang_test_dir = root_dir
            .join("fixtures")
            .join("test-cases")
            .join("e2e")
            .join("rust")
            .join("rust.rs.txt");
        let code = read_code(&lang_test_dir);

        let grammar = Grammar::from_code("extensions/rust/syntaxes/rust.tmLanguage.json", &*code);
        assert_eq!(grammar.rule_id2desc.len(), 76);
    }
}
