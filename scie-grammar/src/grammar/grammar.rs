use std::collections::BTreeMap as Map;

use crate::grammar::line_tokens::{LineTokens, TokenTypeMatcher};
use crate::grammar::local_stack_element::LocalStackElement;
use crate::grammar::{MatchRuleResult, ScopeListElement, StackElement};
use crate::inter::{IRawGrammar, IRawRepository, IRawRepositoryMap, IRawRule};
use crate::rule::abstract_rule::RuleEnum;
use crate::rule::rule_factory::RuleFactory;
use crate::rule::{
    AbstractRule, EmptyRule, IGrammarRegistry, IRuleFactoryHelper,
    IRuleRegistry,
};
use core::cmp;
use scie_scanner::scanner::scanner::{IOnigCaptureIndex};

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

    fn tokenize(
        &mut self,
        line_text: String,
        prev_state: Option<StackElement>,
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

        let mut current_state = StackElement::null();

        match prev_state.clone() {
            None => is_first_line = true,
            Some(state) => {
                if state == StackElement::null() {
                    is_first_line = true
                }

                current_state = state;
            }
        }

        if is_first_line {
            // let scope_list = ScopeListElement::default();
            let _root_scope_name = self.get_rule(self.root_id.clone()).get_name(None, None);
            let mut root_scope_name = String::from("unknown");
            if let Some(name) = _root_scope_name {
                root_scope_name = name
            }

            let scope_list = ScopeListElement::new(None, root_scope_name);
            let mut state = StackElement::new(
                None,
                self.root_id.clone(),
                -1,
                -1,
                false,
                None,
                scope_list.clone(),
                scope_list.clone(),
            );

            current_state = state;
        } else {
            is_first_line = false;
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
            &mut current_state,
            line_tokens,
            true,
        );
    }

    pub fn tokenize_string(
        &mut self,
        line_text: String,
        origin_is_first: bool,
        origin_line_pos: i32,
        stack: &mut StackElement,
        mut line_tokens: LineTokens,
        check_while_conditions: bool,
    ) -> Option<StackElement> {
        let _line_length = line_text.len();
        let mut _stop = false;
        let mut anchor_position = -1;

        if check_while_conditions {
            // todo: add realy logic
            self.check_while_conditions(
                line_text.clone(),
                origin_is_first.clone(),
                origin_line_pos.clone(),
                stack.clone(),
                line_tokens.clone(),
            );
        }

        let mut line_pos = origin_line_pos.clone();
        let mut is_first_line = origin_is_first.clone();
        while !_stop {
            let r = self.match_rule(
                line_text.clone(),
                is_first_line,
                line_pos,
                stack,
                anchor_position,
            );
            if let None = r {
                _stop = true;
                return None;
            }

            let capture_result = r.unwrap();
            let capture_indices = capture_result.capture_indices;
            let matched_rule_id = capture_result.matched_rule_id;
            if matched_rule_id == -1 {
                println!("todo: matched the `end` for this rule => pop it");
            } else {
                let rule = self.get_rule(matched_rule_id);
                line_tokens.produce(stack, capture_indices[0].start as i32);
                // let before_push = stack.clone();
                let scope_name =
                    rule.get_name(Some(line_text.clone()), Some(capture_indices.clone()));
                let name_scopes_list = stack
                    .content_name_scopes_list
                    .clone()
                    .push(self, scope_name);
                let mut begin_rule_capture_eol = false;
                if capture_indices[0].end == _line_length {
                    begin_rule_capture_eol = true;
                }
                let mut new_stack = stack.clone().push(
                    matched_rule_id,
                    line_pos,
                    anchor_position,
                    begin_rule_capture_eol,
                    None,
                    name_scopes_list.clone(),
                    name_scopes_list.clone(),
                );

                match rule.get_rule_instance() {
                    RuleEnum::BeginEndRule(begin_rule) => {
                        // Grammar::handle_captures(self, line_text.clone(), is_first_line, &mut new_stack, line_tokens.clone(), begin_rule.begin_captures, capture_indices.clone());
                    }
                    RuleEnum::BeginWhileRule(while_rule) => {}
                    _ => {}
                }
            }

            if capture_indices[0].end > line_pos as usize {
                line_pos = capture_indices[0].end as i32;
                is_first_line = false;
            }
        }
        Some(stack.clone())
    }

    pub fn handle_captures(
        grammar: &mut Grammar,
        line_text: String,
        is_first_line: bool,
        stack: &mut StackElement,
        mut line_tokens: LineTokens,
        captures: Vec<Box<dyn AbstractRule>>,
        capture_indices: Vec<IOnigCaptureIndex>,
    ) {
        let captures_len = captures.clone().len();
        if captures_len == 0 {
            return;
        }

        let len = cmp::min(captures_len, capture_indices.len());
        let mut local_stack: Vec<LocalStackElement> = vec![];
        let max_end = capture_indices[0].end;
        for i in 0..len {
            let capture_rule = captures[i].clone();
            // if let None = capture_rule {
            //     continue
            // }

            let capture_index = capture_indices[i].clone();
            if capture_index.length == 0 {
                continue;
            }

            if capture_index.start > max_end {
                continue;
            }

            while local_stack.len() > 0
                && local_stack[local_stack.len() - 1].end_pos <= capture_index.start as i32
            {
                let mut local_stack_element = local_stack[local_stack.len() - 1].clone();
                line_tokens.produce_from_scopes(
                    &mut local_stack_element.scopes,
                    local_stack_element.end_pos,
                );
                local_stack.pop();
            }

            if local_stack.len() > 0 {
                let mut local_stack_element = local_stack[local_stack.len() - 1].clone();
                line_tokens.produce_from_scopes(
                    &mut local_stack_element.scopes,
                    local_stack_element.end_pos,
                );
            } else {
                line_tokens.produce(stack, capture_index.start as i32);
            }
        }
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
        stack: &mut StackElement,
        anchor_position: i32,
    ) {
        let match_result =
            self.match_rule(line_text, is_first_line, line_pos, stack, anchor_position);
        if let Some(result) = match_result {
        } else {
            // None
        };
        // todo: get injections logic
    }

    pub fn match_rule(
        &mut self,
        line_text: String,
        is_first_line: bool,
        line_pos: i32,
        stack: &mut StackElement,
        anchor_position: i32,
    ) -> Option<MatchRuleResult> {
        let mut rule = stack.get_rule(self);
        let mut rule_scanner = rule.compile(
            self,
            stack.end_rule.clone(),
            is_first_line,
            line_pos == anchor_position,
        );
        let r = rule_scanner
            .scanner
            .find_next_match_sync(line_text, line_pos);
        if let Some(result) = r {
            let match_rule_result = MatchRuleResult {
                capture_indices: result.capture_indices,
                matched_rule_id: rule_scanner.rules[result.index],
            };

            println!("{:?}", match_rule_result.clone());
            Some(match_rule_result)
        } else {
            None
        }
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
    use crate::rule::IRuleRegistry;

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
        assert_eq!(grammar.rule_id2desc.len(), 67);
        debug_output(&grammar, String::from("program.json"));
    }

    #[test]
    fn should_build_makefile_grammar() {
        let code = "CC=gcc
CFLAGS=-I.
DEPS = hellomake.h
OBJ = hellomake.o hellofunc.o

%.o: %.c $(DEPS)
	$(CC) -c -o $@ $< $(CFLAGS)

hellomake: $(OBJ)
	$(CC) -o $@ $^ $(CFLAGS)
";
        let mut grammar = to_grammar("test-cases/first-mate/fixtures/makefile.json", code);
        assert_eq!(grammar.rule_id2desc.len(), 64);
        assert_eq!(grammar.get_rule(1).patterns_length(), 4);
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
