use std::collections::BTreeMap as Map;

use onig::*;

use crate::grammar::line_tokens::{LineTokens, TokenTypeMatcher};
use crate::grammar::{ScopeListElement, StackElement};
use crate::inter::{IRawGrammar, IRawRepository, IRawRepositoryMap, IRawRule};
use crate::rule::rule_factory::RuleFactory;
use crate::rule::{AbstractRule, EmptyRule, IGrammarRegistry, IRuleFactoryHelper, IRuleRegistry};

pub struct IToken {
    pub start_index: i32,
    pub end_index: i32,
    pub scopes: Vec<String>,
}

pub struct ITokenizeLineResult {
    pub tokens: Vec<IToken>,
    pub rule_stack: Box<StackElement>,
}

pub struct ITokenizeLineResult2 {
    pub tokens: Vec<i32>,
    pub rule_stack: Box<StackElement>,
}

pub trait IGrammar {
    fn tokenize_line(line_text: String, prev_state: Option<StackElement>) -> ITokenizeLineResult;
    /**
     * Tokenize `lineText` using previous line state `prevState`.
     * The result contains the tokens in binary format, resolved with the following information:
     *  - language
     *  - token type (regex, string, comment, other)
     *  - font style
     *  - foreground color
     *  - background color
     * e.g. for getting the languageId: `(metadata & MetadataConsts.LANGUAGEID_MASK) >>> MetadataConsts.LANGUAGEID_OFFSET`
     */
    fn tokenize_line2(line_text: String, prev_state: Option<StackElement>) -> ITokenizeLineResult2;
}

pub trait Matcher {}

#[derive(Debug, Clone)]
pub struct Grammar {
    root_id: i32,
    grammar: IRawGrammar,
    pub last_rule_id: i32,
    pub rule_id2desc: Map<i32, Box<dyn AbstractRule>>,
    pub _token_type_matchers: Vec<TokenTypeMatcher>,
}

pub fn init_grammar(grammar: IRawGrammar, _base: Option<IRawRule>) -> IRawGrammar {
    let mut _grammar = grammar.clone();

    let mut new_based: IRawRule = IRawRule::new();
    if let Some(repo) = grammar.clone().repository {
        new_based.location = repo.clone().location;
    }
    new_based.patterns = Some(grammar.clone().patterns.clone());
    new_based.name = grammar.clone().name;

    let mut repository_map = IRawRepositoryMap::new();
    repository_map.base_s = Some(new_based.clone());
    repository_map.self_s = Some(new_based.clone());
    if let Some(repo) = grammar.clone().repository {
        repository_map.name_map = repo.clone().map.name_map.clone();
    }

    _grammar.repository = Some(IRawRepository {
        map: Box::new(repository_map.clone()),
        location: None,
    });

    _grammar
}

impl Grammar {
    pub fn new(grammar: IRawGrammar) -> Grammar {
        let _grammar = init_grammar(grammar.clone(), None);
        Grammar {
            last_rule_id: 0,
            grammar: _grammar,
            root_id: -1,
            rule_id2desc: Map::new(),
            _token_type_matchers: vec![],
        }
    }
    // todo: refactor to callback ??
    pub fn create_onig_scanner(&self, sources: String) -> Regex {
        Regex::new(sources.as_str()).unwrap()
    }
    pub fn create_onig_string(&self, sources: String) -> String {
        sources
    }

    fn tokenize(
        &mut self,
        line_text: String,
        mut prev_state: Option<StackElement>,
        emit_binary_tokens: bool,
    ) {
        if self.root_id.clone() == -1 {
            let mut repository = self.grammar.repository.clone().unwrap();
            let based = repository.clone().map.self_s.unwrap();
            self.root_id = RuleFactory::get_compiled_rule_id(
                based.clone(),
                self,
                &mut repository.clone(),
                String::from(""),
            );
        }

        let mut is_first_line: bool = false;
        match prev_state.clone() {
            None => is_first_line = true,
            Some(state) => {
                if state == StackElement::null() {
                    is_first_line = true
                }
            }
        }

        if is_first_line {
            let scope_list = ScopeListElement::default();
            prev_state = Some(StackElement::new(
                None,
                self.root_id.clone(),
                -1,
                -1,
                false,
                None,
                scope_list.clone(),
                scope_list.clone(),
            ))
        }

        let format_line_text = format!("{:?}\n", line_text);
        let line_tokens = LineTokens::new(
            emit_binary_tokens,
            line_text,
            self._token_type_matchers.clone(),
        );
        self.tokenize_string(
            format_line_text.parse().unwrap(),
            is_first_line,
            0,
            prev_state.unwrap(),
            line_tokens,
            true,
        )
    }

    pub fn tokenize_string(
        &mut self,
        line_text: String,
        is_first_line: bool,
        line_pos: i32,
        prev_state: StackElement,
        line_tokens: LineTokens,
        check_while_conditions: bool,
    ) {
        let _line_length = line_text.len();
        let _stop = false;
        let mut anchor_position = -1;

        if check_while_conditions {
            // todo: add realy logic
            self.check_while_conditions(
                line_text.clone(),
                is_first_line.clone(),
                line_pos.clone(),
                prev_state.clone(),
                line_tokens.clone(),
            );
        }

        self.match_rule_or_injections(
            line_text,
            is_first_line,
            line_pos,
            prev_state,
            anchor_position,
        );
    }

    pub fn check_while_conditions(
        &mut self,
        line_text: String,
        is_first_line: bool,
        line_pos: i32,
        _stack: StackElement,
        line_tokens: LineTokens,
    ) {
        let mut anchor_position = -1;
        if _stack.begin_rule_captured_eol {
            anchor_position = 0
        }
        // let while_rules = vec![];
    }

    pub fn match_rule_or_injections(
        &mut self,
        line_text: String,
        is_first_line: bool,
        line_pos: i32,
        stack: StackElement,
        anchor_position: i32,
    ) {
        self.match_rule(
            line_text,
            is_first_line,
            line_pos,
            stack.clone(),
            anchor_position,
        );
    }

    pub fn match_rule(
        &mut self,
        line_text: String,
        is_first_line: bool,
        line_pos: i32,
        stack: StackElement,
        anchor_position: i32,
    ) {
        let mut rule = stack.get_rule(self);
        rule.compile(
            self,
            stack.end_rule,
            is_first_line,
            line_pos == anchor_position,
        );
        println!("{:?}", rule.type_of());
    }

    pub fn tokenize_line(&mut self, line_text: String, prev_state: Option<StackElement>) {
        self.tokenize(line_text, prev_state, false)
    }

    pub fn tokenize_line2(&self, line_text: String, prev_state: Option<StackElement>) {}
}

impl IRuleFactoryHelper for Grammar {}

impl IGrammarRegistry for Grammar {
    fn get_external_grammar(
        &self,
        scope_name: String,
        repository: IRawRepository,
    ) -> Option<IRawGrammar> {
        None
    }
}

impl IRuleRegistry for Grammar {
    fn register_id(&mut self) -> i32 {
        self.last_rule_id = self.last_rule_id + 1;
        self.last_rule_id.clone()
    }

    fn get_rule(&mut self, pattern_id: i32) -> Box<dyn AbstractRule> {
        if let Some(rule) = self.rule_id2desc.get_mut(&pattern_id) {
            return rule.to_owned();
        }
        Box::from(EmptyRule {})
    }

    fn register_rule(&mut self, result: Box<dyn AbstractRule>) -> Box<dyn AbstractRule> {
        self.rule_id2desc
            .insert(result.id().clone(), result.clone());
        result
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{Read, Write};
    use std::path::Path;

    use crate::grammar::Grammar;
    use crate::inter::IRawGrammar;

    #[test]
    fn should_build_json_code() {
        let code = "
#include <stdio.h>
int main() {
printf(\"Hello, World!\");
return 0;
}
";
        let grammar = to_grammar("test-cases/first-mate/fixtures/c.json", code);
        // todo: fixed it
        // assert_eq!(grammar.rule_id2desc.len(), 162);
        // debug_output(&grammar, String::from("program.json"));
    }

    #[test]
    fn should_build_text_grammar() {
        let code = "
GitHub 漫游指南
";
        let grammar = to_grammar("test-cases/first-mate/fixtures/text.json", code);
        assert_eq!(grammar.rule_id2desc.len(), 8);
    }

    fn debug_output(grammar: &Grammar, path: String) {
        let j = serde_json::to_string(&grammar.rule_id2desc).unwrap();
        let mut file = File::create(path).unwrap();
        match file.write_all(j.as_bytes()) {
            Ok(_) => {}
            Err(_) => {}
        };
    }

    #[test]
    fn should_build_json_grammar() {
        let code = "{}";
        let grammar = to_grammar("test-cases/first-mate/fixtures/json.json", code);
        assert_eq!(grammar.rule_id2desc.len(), 22);
        debug_output(&grammar, String::from("program.json"));
    }

    #[test]
    fn should_build_html_grammar() {
        let code = "{}";
        let grammar = to_grammar("test-cases/first-mate/fixtures/html.json", code);
        // assert_eq!(grammar.rule_id2desc.len(), 104);
        assert_eq!(grammar.rule_id2desc.len(), 67);
        debug_output(&grammar, String::from("program.json"));
    }

    #[test]
    fn should_build_makefile_grammar() {
        let code = "{}";
        let grammar = to_grammar("test-cases/first-mate/fixtures/makefile.json", code);
        // assert_eq!(grammar.rule_id2desc.len(), 82);
        assert_eq!(grammar.rule_id2desc.len(), 64);
        debug_output(&grammar, String::from("program.json"));
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
