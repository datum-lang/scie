use crate::inter::IRawGrammar;

pub mod inter;

fn parse_raw_grammar(content: String, file_path: Option<String>) -> IRawGrammar {
    if let Some(path) = file_path.clone() {
        if path.ends_with(".json") {
            return parse_json_grammar(content, path)
        }
    }

    return parse_plist_grammar(content, file_path.clone())
}

// todo: in current, we don't need to impl it
fn parse_plist_grammar(content: String, file_path: Option<String>) -> IRawGrammar {
    IRawGrammar::new()
}

fn parse_json_grammar(content: String, file_path: String) -> IRawGrammar {
    IRawGrammar::new()
}


#[cfg(test)]
mod tests {
    use crate::parse_raw_grammar;

    #[test]
    fn should_run() {
        let grammar = parse_raw_grammar(String::from("hello"), Some(String::from("world.json")));
        assert_eq!(format!("{:?}", grammar.location), "ILocatable { textmate_location: None }");
    }
}