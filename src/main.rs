#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

use std::path::PathBuf;
use std::fs::File;
use std::io::Read;

pub mod artifact;
pub mod bindata;

lazy_static! {
  static ref DEFAULT_VCS_EXCLUDES: Vec<&'static str> = vec![
        "**/%*%",
        "**/.#*",
        "**/._*",
        "**/#*#",
        "**/*~",
        "**/.DS_Store",

        "**/CVS",
        "**/CVS/**",
        "**/.cvsignore",

        "**/SCCS",
        "**/SCCS/**",

        "**/.bzr",
        "**/.bzr/**",
        "**/.bzrignore",

        "**/vssver.scc",

        "**/.hg",
        "**/.hg/**",
        "**/.hgtags",
        "**/.hgignore",
        "**/.hgsubstate",
        "**/.hgsub",

        "**/.svn",
        "**/.svn/**",

        "**/.git",
        "**/.git/**",
        "**/.gitignore",
        "**/.gitmodules",
        "**/.gitattributes"
    ];

}

fn main() {}

fn read_code(lang_test_dir: &PathBuf) -> String {
    let mut file = File::open(lang_test_dir).unwrap();
    let mut code = String::new();
    file.read_to_string(&mut code).unwrap();
    code
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use scie_grammar::grammar::{Grammar, StackElement};
    use crate::artifact::{Element};
    use crate::read_code;

    #[test]
    fn should_build_first_file() {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).to_path_buf();
        let lang = root_dir.clone()
            .join("extensions").join("java").join("syntaxes").join("java.tmLanguage.json");

        let code_dir = root_dir.join("fixtures").join("test-cases").join("e2e").join("java").join("HelloWorld.java");
        let code = read_code(&code_dir);

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

                elements.push(
                    Element {
                        line_num,
                        start_index: start,
                        end_index: end,
                        value: text,
                        scopes: vec![],
                    }
                );
            }

            rule_stack = result.rule_stack;
            line_num = line_num + 1
        }

        println!("{:?}", elements);
    }
}