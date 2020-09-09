use std::env;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use scie_grammar::grammar::{Grammar, StackElement};

fn main() {
    let target_dir = get_target_dir();
    let root_dir = get_top_dir(&*target_dir);

    let lang_spec_dir = root_dir.join("extensions").join("json").join("syntaxes").join("JSON.tmLanguage.json");
    let lang_test_dir = Path::new("fixtures").join("JavaScript.tmLanguage.json.txt");

    let code = read_code(&lang_test_dir);

    let mut grammar = Grammar::to_grammar(lang_spec_dir.to_str().unwrap());

    let mut rule_stack = Some(StackElement::null());

    let start = SystemTime::now();
    for line in code.lines() {
        // println!("{:?}", line);
        let result = grammar.tokenize_line(String::from(line), &mut rule_stack);
        rule_stack = *result.rule_stack;
    }

    if let Ok(n) = SystemTime::now().duration_since(start) {
        println!("TOKENIZING {:?} length using grammar source.js {:?} ms", code.len(), n.as_millis())
    }
}

fn read_code(lang_test_dir: &PathBuf) -> String {
    let mut file = File::open(lang_test_dir).unwrap();
    let mut code = String::new();
    file.read_to_string(&mut code).unwrap();
    code
}

// https://github.com/rust-lang/cargo/issues/2841
fn get_target_dir() -> PathBuf {
    let bin = env::current_exe().expect("exe path");
    let mut target_dir = PathBuf::from(bin.parent().expect("bin parent"));
    while target_dir.file_name() != Some(OsStr::new("target")) {
        target_dir.pop();
    }
    target_dir
}

fn get_top_dir<'a>(target_dir: &'a Path) -> &'a Path {
    target_dir.parent().expect("target parent")
}
