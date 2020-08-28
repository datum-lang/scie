use std::collections::BTreeMap as Map;
use crate::rule::{AbstractRule, MatchRule, IncludeOnlyRule, CaptureRule, BeginWhileRule, BeginEndRule};

#[derive(Serialize, Debug)]
pub struct RuleList {
    #[serde(flatten)]
    match_rule: Option<MatchRule>,

    #[serde(flatten)]
    include_only_rule: Option<IncludeOnlyRule>,

    #[serde(flatten)]
    capture: Option<CaptureRule>,

    #[serde(flatten)]
    begin_while_rule: Option<BeginWhileRule>,

    #[serde(flatten)]
    begin_end_rule: Option<BeginEndRule>,
}

// todo: Convert generic trait T back to struct
// https://users.rust-lang.org/t/convert-generic-trait-t-back-to-struct/11581
fn abstract_rule_to_json(map: Map<i32, Box<dyn AbstractRule>>) {
    let mut rule_list: Vec<RuleList> = vec![];
    // for (key, value) in map {
    //     if value.type_of() == "BeginEndRule" {
    //         rule_list.push(RuleList {
    //             match_rule: None,
    //             include_only_rule: None,
    //             capture: None,
    //             begin_while_rule: None,
    //             begin_end_rule: Some(*value as BeginEndRule),
    //         })
    //     } else if value.type_of() == "BeginWhileRule" {
    //         rule_list.push(RuleList {
    //             match_rule: None,
    //             include_only_rule: None,
    //             capture: None,
    //             begin_end_rule: None,
    //             begin_while_rule: Some(*value as BeginWhileRule),
    //         })
    //     }
    // }

    let j = serde_json::to_string(&rule_list).unwrap();
    println!("{:?}", j);
}


#[cfg(test)]
mod tests {
    use crate::grammar::grammar::Grammar;
    use std::path::Path;
    use std::fs::File;
    use crate::inter::IRawGrammar;
    use std::io::Read;
    use crate::rule::rule_convert::abstract_rule_to_json;

    #[test]
    fn should_build_text_grammar() {
        let code = "
GitHub 漫游指南
";
        let grammar = to_grammar("test-cases/first-mate/fixtures/text.json", code);
        abstract_rule_to_json(grammar.rule_id2desc);
    }

    fn to_grammar(grammar_path: &str, code: &str) -> Grammar {
        let path = Path::new(grammar_path);
        let mut file = File::open(path).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        let g: IRawGrammar = serde_json::from_str(&data).unwrap();

        let mut grammar = Grammar::new(g);
        let c_code = String::from(code);
        for line in c_code.lines() {
            grammar.tokenize_line(String::from(line), None)
        }

        grammar
    }
}