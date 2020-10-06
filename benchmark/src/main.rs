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

    run(root_dir.clone(), "json", "JSON.tmLanguage.json", "JavaScript.tmLanguage.json.txt");
    // run(root_dir.clone(), "javascript", "JavaScript.tmLanguage.json", "large.min.js.txt");
    // run(root_dir.clone(), "javascript", "JavaScript.tmLanguage.json", "large.js.txt");
}

fn run(root_dir: PathBuf, lang: &str, lang_file: &str, code_file: &str) {
    let lang_spec_dir = root_dir
        .clone()
        .join("extensions")
        .join(lang)
        .join("syntaxes")
        .join(lang_file);
    let code_dir = root_dir
        .join("benchmark")
        .join("samples")
        .join(code_file);
    let code = read_code(&code_dir);

    run_execute(lang_spec_dir, code)
}

fn run_execute(lang_spec_dir: PathBuf, code: String) {
    let mut grammar = Grammar::from_file(lang_spec_dir.to_str().unwrap());

    let mut rule_stack = Some(StackElement::null());
    let start = SystemTime::now();

    for line in code.lines() {
        let result = grammar.tokenize_line(line, &mut rule_stack);
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
