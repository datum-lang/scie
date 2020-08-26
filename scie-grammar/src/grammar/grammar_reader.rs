use crate::inter::IRawGrammar;

fn parse_raw_grammar(content: String, file_path: Option<String>) -> Result<IRawGrammar, String> {
    if let Some(path) = file_path.clone() {
        if path.ends_with(".json") {
            return Ok(parse_json_grammar(content, path));
        }
    }

    Err(String::from("only support json file, input file is not a json file"))
}

fn parse_json_grammar(content: String, file_path: String) -> IRawGrammar {
    IRawGrammar::new()
}

#[cfg(test)]
mod tests {
    use crate::grammar::grammar_reader::parse_raw_grammar;

    #[test]
    fn should_run() {
        let grammar = parse_raw_grammar(String::from("hello"), Some(String::from("world.json")));
        assert_eq!(
            format!("{:?}", grammar.unwrap().location),
            "ILocatable { textmate_location: None }"
        );
    }
}
