use std::path::PathBuf;

use scie_grammar::grammar::{Grammar, StackElement};
use scie_model::artifact::TokenElement;

pub struct Identify {}

impl Identify {
    pub fn identify_file(lang: PathBuf, code: String) -> Vec<TokenElement> {
        let mut elements: Vec<TokenElement> = vec![];
        let mut grammar = Grammar::from_file(lang.to_str().unwrap());
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

                let element = TokenElement::new(line_num, start, end, text, token.scopes);
                elements.push(element);
            }

            rule_stack = result.rule_stack;
            line_num = line_num + 1
        }

        elements
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::analyser::Identify;
    use scie_infra::finder::Finder;

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
        let code = Finder::read_code(&code_dir).unwrap();

        let elements = Identify::identify_file(lang, code);
        assert_eq!(39, elements.len());
    }
}
