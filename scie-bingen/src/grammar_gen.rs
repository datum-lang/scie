use crate::language_gen::LangExtGen;
use scie_grammar::inter::IRawGrammar;
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct LangConfig {
    pub path: String,
    pub lang: String,
    pub scope_name: String,
}

impl LangConfig {}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct GrammarGen {
    grammar_map: HashMap<String, IRawGrammar>,
}

impl GrammarGen {
    pub fn new() -> Self {
        GrammarGen {
            grammar_map: Default::default(),
        }
    }

    pub fn build_output() -> GrammarGen {
        let config_map = GrammarGen::build_grammar_map();
        let mut grammar_map: HashMap<String, IRawGrammar> = Default::default();
        for (lang, config) in config_map {
            let path = Path::new(&config.path);
            let mut file = File::open(path).unwrap();
            let mut data = String::new();
            file.read_to_string(&mut data).unwrap();

            match serde_json::from_str(&data) {
                Ok(x) => {
                    grammar_map.insert(lang, x);
                }
                Err(err) => panic!(err),
            };
        }

        GrammarGen { grammar_map }
    }

    pub fn build_grammar_map() -> BTreeMap<String, LangConfig> {
        let langs = LangExtGen::default();
        let mut raw_grammar_map: BTreeMap<String, LangConfig> = Default::default();
        for (ext, entry) in langs.ext_map.iter() {
            let lang = entry.name.clone();
            let tm_grammar = langs.grammar_map[&lang].clone();

            let mut grammar_path = entry.path.clone();
            grammar_path.push_str(&*tm_grammar.path);
            grammar_path = grammar_path.replace("./", "/");

            if tm_grammar.language.is_some() {
                raw_grammar_map.insert(
                    String::from(ext),
                    LangConfig {
                        path: grammar_path,
                        lang: tm_grammar.language.unwrap(),
                        scope_name: tm_grammar.scope_name,
                    },
                );
            }
        }

        raw_grammar_map
    }

    fn to_json_file(&self, path: &str) {
        let json_str = serde_json::to_string_pretty(&self).unwrap();
        let bytes = json_str.as_bytes();

        let mut file = File::create(path).unwrap();
        match file.write_all(bytes) {
            Ok(_) => {}
            Err(_) => {}
        };
    }

    fn to_bin_file(&self, path: &str) {
        let encoded: Vec<u8> = bincode::serialize(&self).unwrap();
        let mut file = File::create(path).unwrap();
        match file.write_all(&*encoded) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::grammar_gen::GrammarGen;
    use scie_grammar::grammar::{Grammar, StackElement};

    #[test]
    fn should_build_default_maps() {
        let map = GrammarGen::build_grammar_map();
        assert!(map[".zshrc"]
            .path
            .contains("extensions/shellscript/syntaxes/shell-unix-bash.tmLanguage.json"));
    }

    #[test]
    fn should_build_grammar_gen() {
        let map = GrammarGen::build_output();
        // map.to_bin_file("grammar.bin");
        map.to_json_file("grammar.json");
    }

    #[test]
    fn should_use_grammar_gen_map_build_grammar() {
        let map = GrammarGen::build_grammar_map();
        let groovy_path = map[".gradle"].path.clone();

        let mut grammar = Grammar::from_file(&*groovy_path);
        let mut rule_stack = Some(StackElement::null());

        let result = grammar.tokenize_line("println \"hello, world!\"", &mut rule_stack);
        assert_eq!(5, result.tokens.len());
    }
}
