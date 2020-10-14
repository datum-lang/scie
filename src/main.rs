extern crate serde_derive;

extern crate serde;

use std::path::PathBuf;
use scie_infra::finder::Finder;
use scie_bingen::grammar_gen::GrammarGen;
use scie_grammar::grammar::{Grammar, StackElement};
use scie_detector::framework_detector::FrameworkDetector;
use std::collections::HashMap;

pub mod identify;

fn main() {}

pub fn get_lang_by_path(path: PathBuf) -> String {
    let mut str = ".".to_owned();
    let ext = path.extension().unwrap().to_str().unwrap();
    str.push_str(ext);

    str
}

pub fn ident_by_dir(lang: &PathBuf) {
    let mut detector = FrameworkDetector::new();
    detector.run(lang.display().to_string());

    let files = Finder::walk_filter_files(&lang);
    let map = GrammarGen::build_output();

    let mut grammar_map = HashMap::new();
    if detector.tags.contains_key("workspace.java.gradle") {
        grammar_map.insert(".groovy", Grammar::new(map.grammar_map[".groovy"].clone()));
        grammar_map.insert(".java", Grammar::new(map.grammar_map[".java"].clone()));
    }

    if detector.tags.contains_key("workspace.rust.cargo") {
        grammar_map.insert(".rust", Grammar::new(map.grammar_map[".rs"].clone()));
    }

    for path in files {
        if path.extension().is_none() { continue; }
        println!("analyses: {:?}", path);

        let lang = get_lang_by_path(path.clone());
        let lang_grammar = grammar_map.get_mut(lang.as_str());
        if lang_grammar.is_none() {
            continue;
        }

        let grammar = lang_grammar.unwrap();

        let code = Finder::read_code(&path);
        let mut rule_stack = Some(StackElement::null());
        for line in code.lines() {
            let result = grammar.tokenize_line(line, &mut rule_stack);
            rule_stack = result.rule_stack;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::identify::Identify;
    use scie_infra::finder::Finder;
    use std::path::PathBuf;
    use crate::ident_by_dir;

    #[test]
    fn should_build_first_file() {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).to_path_buf();
        let lang = root_dir
            .clone()
            .join("extensions")
            .join("java")
            .join("syntaxes")
            .join("java.tmLanguage.json");

        let code_dir = root_dir
            .join("fixtures")
            .join("test-cases")
            .join("e2e")
            .join("java")
            .join("HelloWorld.java");
        let code = Finder::read_code(&code_dir);

        let elements = Identify::identify_file(lang, code);
        assert_eq!(39, elements.len());
    }

    #[test]
    fn should_identify_path() {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).to_path_buf();
        let lang = root_dir
            .clone()
            .join("fixtures")
            .join("projects")
            .join("java")
            .join("simple");

        ident_by_dir(&lang)
    }

    #[test]
    fn should_identify_self_grammar() {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).to_path_buf();
        let lang = root_dir
            .clone()
            .join("scie-grammar");

        ident_by_dir(&lang)
    }
}
