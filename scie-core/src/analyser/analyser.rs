use std::collections::HashMap;
use std::path::PathBuf;

use scie_bingen::grammar_gen::GrammarGen;
use scie_detector::framework_detector::FrameworkDetector;
use scie_grammar::grammar::{Grammar, StackElement};
use scie_infra::finder::Finder;
use scie_model::artifact::{CodeFile, TokenElement};

pub struct Analyser {}

impl Analyser {
    pub fn get_lang_by_path(path: PathBuf) -> String {
        let mut str = ".".to_owned();
        let ext = path.extension().unwrap().to_str().unwrap();
        str.push_str(ext);

        str
    }

    pub fn ident_by_dir(lang: &PathBuf, is_debug: bool, is_cli: bool) -> Vec<CodeFile> {
        let mut detector = FrameworkDetector::new();
        detector.run(lang.display().to_string());

        let map = GrammarGen::build_output();

        let mut grammar_map = HashMap::new();
        if detector.tags.contains_key("workspace.java.gradle") {
            let groovy_grammar = Grammar::new(map.grammar_map[".groovy"].clone());
            grammar_map.insert(".gradle", groovy_grammar.clone());
            grammar_map.insert(".groovy", groovy_grammar);

            let java_grammar = Grammar::new(map.grammar_map[".java"].clone());
            grammar_map.insert(".java", java_grammar);
        }

        if detector.tags.contains_key("workspace.rust.cargo") {
            let rust_grammar = Grammar::new(map.grammar_map[".rs"].clone());
            grammar_map.insert(".rs", rust_grammar);
        }

        if detector.tags.contains_key("workspace.go") {
            let c_grammar = Grammar::new(map.grammar_map[".go"].clone());
            grammar_map.insert(".go", c_grammar);
        }

        if detector.tags.contains_key("workspace.c") {
            let c_grammar = Grammar::new(map.grammar_map[".c"].clone());
            grammar_map.insert(".c", c_grammar);
        }

        let files = Finder::walk_filter_files(&lang);
        if is_debug {
            if !detector.tags.is_empty() {
                println!("{:?}", detector.tags);
            }
        }

        Analyser::process_files(&mut grammar_map, files, is_debug, is_cli)
    }

    fn process_files(
        grammar_map: &mut HashMap<&str, Grammar>,
        files: Vec<PathBuf>,
        _is_debug: bool,
        is_cli: bool,
    ) -> Vec<CodeFile> {
        let mut parsed_files = vec![];
        for path in files {
            if path.extension().is_none() {
                continue;
            }

            if is_cli {
                // todo: add clear current line & set value http://rosettacode.org/wiki/Terminal_control
                println!("analyses: {:?}", path);
            }

            let lang = Analyser::get_lang_by_path(path.clone());
            let lang_grammar = grammar_map.get_mut(lang.as_str());
            if lang_grammar.is_none() {
                continue;
            }

            let grammar = lang_grammar.unwrap();
            let mut code_file = CodeFile::new(path.clone());
            let code;
            match Finder::read_code(&path) {
                Ok(str) => {
                    code = str;
                }
                Err(_) => {
                    continue;
                }
            }
            let mut rule_stack = Some(StackElement::null());

            let mut line_num = 1;
            for line in code.lines() {
                let result = grammar.tokenize_line(line, &mut rule_stack);
                for token in result.tokens {
                    let start = token.start_index;
                    let end = token.end_index;
                    let text: String = String::from(line)
                        .chars()
                        .skip(start as usize)
                        .take((end - start) as usize)
                        .collect();

                    code_file.elements.push(TokenElement::new(
                        line_num,
                        start,
                        end,
                        text,
                        token.scopes,
                    ));
                }
                rule_stack = result.rule_stack;
                line_num = line_num + 1;
            }

            parsed_files.push(code_file);
        }

        parsed_files
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::analyser::Analyser;

    #[test]
    fn should_identify_path() {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).to_path_buf();
        let lang = root_dir
            .clone()
            .parent()
            .unwrap()
            .join("fixtures")
            .join("projects")
            .join("java")
            .join("simple");

        let files = Analyser::ident_by_dir(&lang, false, false);
        assert_eq!(3, files.len())
    }

    #[test]
    fn should_identify_self_grammar() {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).to_path_buf();
        let lang = root_dir.clone().parent().unwrap().join("scie-grammar");

        let _files = Analyser::ident_by_dir(&lang, false, false);
    }
}
