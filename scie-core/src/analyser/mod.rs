use scie_bingen::grammar_gen::GrammarGen;
use scie_detector::framework_detector::FrameworkDetector;
use scie_grammar::grammar::{Grammar, StackElement};
use scie_infra::finder::Finder;
use std::collections::HashMap;
use std::path::PathBuf;
use scie_model::artifact::{CodeFile, Element};

pub struct Analyser {}

impl Analyser {
    pub fn get_lang_by_path(path: PathBuf) -> String {
        let mut str = ".".to_owned();
        let ext = path.extension().unwrap().to_str().unwrap();
        str.push_str(ext);

        str
    }

    pub fn ident_by_dir(lang: &PathBuf) -> Vec<CodeFile> {
        let mut detector = FrameworkDetector::new();
        detector.run(lang.display().to_string());

        let map = GrammarGen::build_output();

        let mut grammar_map = HashMap::new();
        if detector.tags.contains_key("workspace.java.gradle") {
            grammar_map.insert(".groovy", Grammar::new(map.grammar_map[".groovy"].clone()));
            grammar_map.insert(".gradle", Grammar::new(map.grammar_map[".groovy"].clone()));
            grammar_map.insert(".java", Grammar::new(map.grammar_map[".java"].clone()));
        }

        if detector.tags.contains_key("workspace.rust.cargo") {
            grammar_map.insert(".rs", Grammar::new(map.grammar_map[".rs"].clone()));
        }

        let files = Finder::walk_filter_files(&lang);
        Analyser::process_files(&mut grammar_map, files)
    }

    fn process_files(grammar_map: &mut HashMap<&str, Grammar>, files: Vec<PathBuf>) -> Vec<CodeFile> {
        let mut parsed_files = vec![];
        for path in files {
            if path.extension().is_none() {
                continue;
            }
            println!("analyses: {:?}", path);

            let lang = Analyser::get_lang_by_path(path.clone());
            let lang_grammar = grammar_map.get_mut(lang.as_str());
            if lang_grammar.is_none() {
                continue;
            }

            let grammar = lang_grammar.unwrap();
            let mut code_file = CodeFile::new(path.clone());
            let code = Finder::read_code(&path);
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

                    code_file.elements.push(Element::new(
                        line_num,
                        start,
                        end,
                        text,
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
    use crate::analyser::Analyser;
    use crate::identify::Identify;
    use scie_infra::finder::Finder;
    use std::path::PathBuf;

    #[test]
    fn should_build_first_file() {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).to_path_buf();
        let lang = root_dir
            .clone()
            .parent()
            .unwrap()
            .join("extensions")
            .join("java")
            .join("syntaxes")
            .join("java.tmLanguage.json");

        let code_dir = root_dir
            .parent()
            .unwrap()
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
            .parent()
            .unwrap()
            .join("fixtures")
            .join("projects")
            .join("java")
            .join("simple");

        let files = Analyser::ident_by_dir(&lang);
        assert_eq!(3, files.len())
    }

    #[test]
    fn should_identify_self_grammar() {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).to_path_buf();
        let lang = root_dir.clone().parent().unwrap().join("scie-grammar");

        let files = Analyser::ident_by_dir(&lang);
        // println!("{:?}", files);
    }
}
