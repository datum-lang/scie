use scie_grammar::grammar::{Grammar, StackElement};
use scie_model::artifact::Element;
use std::path::PathBuf;

pub struct Identify {}

impl Identify {
    pub fn identify_file(lang: PathBuf, code: String) -> Vec<Element> {
        let mut elements: Vec<Element> = vec![];
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

                elements.push(Element {
                    line_num,
                    start_index: start,
                    end_index: end,
                    value: text,
                    scopes: vec![],
                });
            }

            rule_stack = result.rule_stack;
            line_num = line_num + 1
        }

        elements
    }
}
