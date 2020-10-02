use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::time::SystemTime;

use scie_grammar::grammar::{Grammar, StackElement};

fn main() {
    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    // run_for_json(root_dir);
    run_for_javascript(root_dir);
}

fn run_for_javascript(root_dir: PathBuf) {
    let lang_spec_dir = root_dir
        .clone()
        .join("extensions")
        .join("javascript")
        .join("JavaScript.tmLanguage.json");
    let code_dir = root_dir
        .join("fixtures")
        .join("test-cases")
        .join("onigtests")
        .join("fixtures")
        .join("typescript.js");
    let code = read_code(&code_dir);

    run_execute(lang_spec_dir, code)
}

fn run_for_json(root_dir: PathBuf) {
    let lang_spec_dir = root_dir
        .clone()
        .join("extensions")
        .join("json")
        .join("syntaxes")
        .join("JSON.tmLanguage.json");
    let code_dir = root_dir
        .join("fixtures")
        .join("tmlanguage")
        .join("JavaScript.tmLanguage.json.txt");
    let code = read_code(&code_dir);

    run_execute(lang_spec_dir, code)
}

fn run_execute(lang_spec_dir: PathBuf, code: String) {
    let mut grammar = Grammar::from_file(lang_spec_dir.to_str().unwrap());

    let mut rule_stack = Some(StackElement::null());
    let start = SystemTime::now();

    for line in code.lines() {
        let result = grammar.tokenize_line(String::from(line), &mut rule_stack);
        rule_stack = *result.rule_stack;
    }

    if let Ok(n) = SystemTime::now().duration_since(start) {
        println!(
            "TOKENIZING {:?} length using grammar source.js {:?} ms",
            code.len(),
            n.as_millis()
        )
    }
}

fn read_code(lang_test_dir: &PathBuf) -> String {
    let mut file = File::open(lang_test_dir).unwrap();
    let mut code = String::new();
    file.read_to_string(&mut code).unwrap();
    code
}
