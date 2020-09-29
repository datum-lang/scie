use core::cmp;
use std::collections::BTreeMap as Map;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

use scie_scanner::scanner::scie_scanner::IOnigCaptureIndex;

use crate::grammar::line_tokens::{IToken, LineTokens, TokenTypeMatcher};
use crate::grammar::local_stack_element::LocalStackElement;
use crate::grammar::{MatchRuleResult, ScopeListElement, StackElement};
use crate::inter::{IRawGrammar, IRawRepository, IRawRepositoryMap, IRawRule};
use crate::rule::abstract_rule::RuleEnum;
use crate::rule::rule_factory::RuleFactory;
use crate::rule::{
    AbstractRule, BeginWhileRule, EmptyRule, IGrammarRegistry, IRuleFactoryHelper, IRuleRegistry,
};

pub trait Matcher {}

#[derive(Debug, Clone)]
pub struct CheckWhileRuleResult {
    pub rule: Box<BeginWhileRule>,
    pub stack: Box<StackElement>,
}

#[derive(Debug, Clone)]
pub struct CheckWhileConditionResult {
    pub stack: Box<StackElement>,
    pub line_pos: i32,
    pub anchor_position: i32,
    pub is_first_line: bool,
}

#[derive(Debug, Clone)]
pub struct TokenizeResult {
    pub tokens: Vec<IToken>,
    pub rule_stack: Box<Option<StackElement>>,
}

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
    repository_map.base_s = Some(Box::from(new_based.clone()));
    repository_map.self_s = Some(Box::from(new_based.clone()));
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
        prev_state: &mut Option<StackElement>,
        emit_binary_tokens: bool,
    ) -> TokenizeResult {
        if self.root_id.clone() == -1 {
            let repository = self.grammar.repository.clone().unwrap();
            let based = repository.clone().map.self_s.unwrap();
            self.root_id = RuleFactory::get_compiled_rule_id(
                *based.clone(),
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
            let _root_scope_name = self.get_rule(self.root_id.clone()).get_name(None, None);
            let mut root_scope_name = String::from("unknown");
            if let Some(name) = _root_scope_name {
                root_scope_name = name
            }

            let scope_list = ScopeListElement::new(None, root_scope_name);
            let state = StackElement::new(
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
            current_state.reset();
        }

        let format_line_text = line_text.clone() + "\n";
        let mut line_tokens = LineTokens::new(
            emit_binary_tokens,
            line_text,
            self._token_type_matchers.clone(),
        );
        let next_state = self.tokenize_string(
            format_line_text.clone(),
            is_first_line,
            0,
            current_state,
            &mut line_tokens,
            true,
        );

        let line_length = format_line_text.clone().len();
        let stack = &mut next_state.clone().unwrap();
        let vec = line_tokens.get_result(stack, line_length as i32);
        TokenizeResult {
            tokens: vec,
            rule_stack: Box::new(next_state.clone()),
        }
    }

    pub fn tokenize_string(
        &mut self,
        line_text: String,
        mut is_first_line: bool,
        origin_line_pos: i32,
        mut stack: StackElement,
        line_tokens: &mut LineTokens,
        check_while_conditions: bool,
    ) -> Option<StackElement> {
        let _line_length = line_text.len();
        let mut _stop = false;
        let mut anchor_position = -1;
        let mut line_pos = origin_line_pos.clone();

        if check_while_conditions {
            let while_check_result = self.check_while_conditions(
                line_text.clone(),
                is_first_line.clone(),
                origin_line_pos.clone(),
                stack.clone(),
                line_tokens.clone(),
            );
            stack = *while_check_result.stack;
            line_pos = while_check_result.line_pos;
            is_first_line = while_check_result.is_first_line;
            anchor_position = while_check_result.anchor_position;
        }

        while !_stop {
            let r = self.match_rule(
                line_text.clone(),
                is_first_line,
                line_pos,
                &mut stack,
                anchor_position,
            );
            if let None = r {
                line_tokens.produce(&mut stack, _line_length as i32);
                _stop = true;
                return Some(stack.clone());
            }

            let capture_result = r.unwrap();
            let capture_indices = capture_result.capture_indices;
            let matched_rule_id = capture_result.matched_rule_id;
            if matched_rule_id == -1 {
                let _popped_rule = stack.get_rule(self);
                if let RuleEnum::BeginEndRule(popped_rule) = _popped_rule.get_rule_instance() {
                    let name_scopes_list = stack.clone().name_scopes_list;
                    line_tokens.produce(&mut stack, capture_indices[0].start.clone() as i32);
                    stack = stack.set_content_name_scopes_list(name_scopes_list);
                    Grammar::handle_captures(
                        self,
                        line_text.clone(),
                        is_first_line,
                        &mut stack,
                        line_tokens,
                        popped_rule.end_captures,
                        capture_indices.clone(),
                    );

                    line_tokens.produce(&mut stack, capture_indices[0].end as i32);
                    let popped = stack.clone();
                    if let Some(_stack) = stack.pop() {
                        stack = _stack;
                    }
                    anchor_position = popped.anchor_pos;
                } else {
                    println!("_popped_rule {:?}", _popped_rule.clone());
                    _stop = true;
                    return Some(stack.clone());
                }
            } else {
                let rule = self.get_rule(matched_rule_id);
                line_tokens.produce(&mut stack, capture_indices[0].start as i32);
                // let before_push = stack.clone();
                let scope_name =
                    rule.get_name(Some(line_text.clone()), Some(capture_indices.clone()));
                let name_scopes_list = stack.content_name_scopes_list.push(self, scope_name);
                let mut begin_rule_capture_eol = false;
                if capture_indices[0].end == _line_length {
                    begin_rule_capture_eol = true;
                }
                stack = stack.push(
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
                        let push_rule = begin_rule.clone();
                        Grammar::handle_captures(
                            self,
                            line_text.clone(),
                            is_first_line,
                            &mut stack,
                            line_tokens,
                            begin_rule.begin_captures,
                            capture_indices.clone(),
                        );

                        line_tokens.produce(&mut stack, capture_indices[0].end.clone() as i32);
                        anchor_position = capture_indices[0].end.clone() as i32;
                        let content_name = push_rule.get_content_name(
                            Some(line_text.clone()),
                            Some(capture_indices.clone()),
                        );
                        let _content_name_scopes_list = name_scopes_list.push(self, content_name);
                        stack = stack.set_content_name_scopes_list(_content_name_scopes_list);
                    }
                    RuleEnum::BeginWhileRule(_while_rule) => {
                        let push_rule = _while_rule.clone();
                        Grammar::handle_captures(
                            self,
                            line_text.clone(),
                            is_first_line,
                            &mut stack,
                            line_tokens,
                            _while_rule.begin_captures,
                            capture_indices.clone(),
                        );

                        line_tokens.produce(&mut stack, capture_indices[0].end.clone() as i32);
                        anchor_position = capture_indices[0].end.clone() as i32;
                        let content_name = push_rule.get_content_name(
                            Some(line_text.clone()),
                            Some(capture_indices.clone()),
                        );
                        let _content_name_scopes_list = name_scopes_list.push(self, content_name);
                        stack = stack.set_content_name_scopes_list(_content_name_scopes_list);
                    }
                    RuleEnum::MatchRule(match_rule) => {
                        Grammar::handle_captures(
                            self,
                            line_text.clone(),
                            is_first_line,
                            &mut stack,
                            line_tokens,
                            match_rule.captures,
                            capture_indices.clone(),
                        );
                        line_tokens.produce(&mut stack, capture_indices[0].end.clone() as i32);
                        if let Some(_stack) = stack.pop() {
                            stack = _stack;
                        }
                    }
                    _ => {
                        panic!("todo: RuleEnum - Others");
                        // _stop = true;
                        // return Some(stack.clone());
                    }
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
        line_tokens: &mut LineTokens,
        captures: Vec<Box<dyn AbstractRule>>,
        capture_indices: Vec<IOnigCaptureIndex>,
    ) -> Option<LineTokens> {
        let captures_len = captures.clone().len();
        if captures_len == 0 {
            return None;
        }

        let len = cmp::min(captures_len, capture_indices.len());
        let mut local_stack: Vec<LocalStackElement> = vec![];
        let max_end = capture_indices[0].end;
        for i in 0..len {
            let capture_rule = captures[i].clone();
            if let RuleEnum::CaptureRule(capture) = capture_rule.get_rule_instance() {
                if capture.clone().rule._type == "" {
                    continue;
                }

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
                        capture_index.start as i32,
                    );
                } else {
                    line_tokens.produce(stack, capture_index.start as i32);
                }

                if capture.retokenize_captured_with_rule_id != 0 {
                    let scope_name =
                        capture.get_name(Some(line_text.clone()), Some(capture_indices.clone()));
                    let name_scopes_list = stack.content_name_scopes_list.push(grammar, scope_name);
                    let content_name = capture
                        .get_content_name(Some(line_text.clone()), Some(capture_indices.clone()));
                    let content_name_scopes_list = name_scopes_list.push(grammar, content_name);

                    let stack_clone = stack.clone().push(
                        capture.retokenize_captured_with_rule_id,
                        capture_index.start.clone() as i32,
                        -1,
                        false,
                        None,
                        name_scopes_list,
                        content_name_scopes_list,
                    );

                    let sub_text = line_text.split_at(capture_index.end).0;
                    let mut sub_is_first_line = false;
                    if is_first_line && capture_index.start == 0 {
                        sub_is_first_line = true;
                    }
                    Grammar::tokenize_string(
                        grammar,
                        String::from(sub_text),
                        sub_is_first_line,
                        capture_index.start as i32,
                        stack_clone,
                        line_tokens,
                        false,
                    );
                    // todo: find the next_text_not_switch_issues
                    continue;
                }

                let capture_scope_name =
                    capture_rule.get_name(Some(line_text.clone()), Some(capture_indices.clone()));
                if let Some(_name) = capture_scope_name.clone() {
                    let mut base = stack.content_name_scopes_list.clone();
                    if local_stack.len() > 0 {
                        base = local_stack[local_stack.len() - 1].scopes.clone();
                    }
                    let capture_rule_scopes_list = base.push(grammar, capture_scope_name.clone());
                    local_stack.push(LocalStackElement::new(
                        capture_rule_scopes_list,
                        capture_index.end as i32,
                    ));
                }
            } else {
                println!("lose rule: {:?}", capture_rule.clone());
            }
        }

        while local_stack.len() > 0 {
            let mut last_stack = local_stack[local_stack.len() - 1].clone();
            line_tokens.produce_from_scopes(&mut last_stack.scopes, last_stack.end_pos);
            local_stack.pop();
        }

        return Some(line_tokens.to_owned());
    }
    /**
     * Walk the stack from bottom to top, and check each while condition in this order.
     * If any fails, cut off the entire stack above the failed while condition. While conditions
     * may also advance the linePosition.
     */
    pub fn check_while_conditions(
        &mut self,
        line_text: String,
        mut is_first_line: bool,
        mut line_pos: i32,
        mut stack: StackElement,
        mut line_tokens: LineTokens,
    ) -> CheckWhileConditionResult {
        let mut anchor_position = -1;
        if stack.begin_rule_captured_eol {
            anchor_position = 0
        }
        let mut while_rules = vec![];
        let mut has_node = true;
        let mut node = stack.clone();
        while has_node {
            let rule = node.clone().get_rule(self);
            if let RuleEnum::BeginWhileRule(begin_rule) = rule.get_rule_instance() {
                while_rules.push(CheckWhileRuleResult {
                    rule: Box::from(begin_rule),
                    stack: Box::from(node.clone()),
                })
            }

            match node.pop() {
                None => has_node = false,
                Some(n) => {
                    node = n;
                }
            }
        }

        for mut while_rule in while_rules.clone() {
            let allow_g = anchor_position == line_pos;
            let mut rule_scanner = while_rule.clone().rule.compile_while(
                self,
                while_rule.clone().stack.end_rule,
                is_first_line,
                allow_g,
            );
            let match_result = rule_scanner
                .scanner
                .find_next_match_sync(line_text.clone(), line_pos);

            match match_result {
                None => {
                    stack = while_rule.stack.pop().unwrap();
                    break;
                }
                Some(r) => {
                    if rule_scanner.rules[r.index] != -2 {
                        stack = while_rule.stack.pop().unwrap();
                        break;
                    }

                    if r.capture_indices.len() > 0 {
                        line_tokens.produce(&mut while_rule.stack, r.capture_indices[0].start as i32);
                        Grammar::handle_captures(self, line_text.clone(), is_first_line,
                                                 &mut *while_rule.stack,
                                                 &mut line_tokens,
                                                 while_rule.rule.while_captures.clone(),
                                                 r.capture_indices.clone(),
                        );
                        line_tokens.produce(&mut while_rule.stack, r.capture_indices[0].end as i32);
                        anchor_position = r.capture_indices[0].end.clone() as i32;
                        if r.capture_indices[0].end > line_pos as usize {
                            line_pos = r.capture_indices[0].end.clone() as i32;
                            is_first_line = false;
                        }
                    }
                }
            }
        }

        // println!("{:?}", while_rules);
        CheckWhileConditionResult {
            stack: Box::new(stack),
            line_pos,
            anchor_position,
            is_first_line,
        }
    }

    // pub fn match_rule_or_injections(
    //     &mut self,
    //     line_text: String,
    //     is_first_line: bool,
    //     line_pos: i32,
    //     stack: &mut StackElement,
    //     anchor_position: i32,
    // ) {
    //     let match_result =
    //         self.match_rule(line_text, is_first_line, line_pos, stack, anchor_position);
    //     if let Some(_result) = match_result {
    //     } else {
    //         // None
    //     };
    //     // todo: get injections logic
    // }

    pub fn match_rule(
        &mut self,
        line_text: String,
        is_first_line: bool,
        line_pos: i32,
        stack: &mut StackElement,
        anchor_position: i32,
    ) -> Option<MatchRuleResult> {
        let mut rule = stack.get_rule(self);
        let rule_info = rule.clone().get_rule_instance();

        let mut rule_scanner;
        match rule_info {
            RuleEnum::BeginEndRule(mut begin_end_rule) => {
                rule_scanner = begin_end_rule.compile(
                    self,
                    stack.end_rule.clone(),
                    is_first_line,
                    line_pos == anchor_position,
                );
            }
            _ => {
                rule_scanner = rule.compile(
                    self,
                    stack.end_rule.clone(),
                    is_first_line,
                    line_pos == anchor_position,
                );
            }
        }

        let r = rule_scanner
            .scanner
            .find_next_match_sync(line_text, line_pos);

        if let Some(result) = r {
            let match_rule_result = MatchRuleResult {
                capture_indices: result.capture_indices,
                matched_rule_id: rule_scanner.rules[result.index],
            };

            Some(match_rule_result)
        } else {
            None
        }
    }

    pub fn tokenize_line(
        &mut self,
        line_text: String,
        prev_state: &mut Option<StackElement>,
    ) -> TokenizeResult {
        self.tokenize(line_text, prev_state, false)
    }

    pub fn from_file(grammar_path: &str) -> Grammar {
        let path = Path::new(grammar_path);
        let mut file = File::open(path).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        let g: IRawGrammar = serde_json::from_str(&data).unwrap();
        Grammar::new(g)
    }
}

impl IRuleFactoryHelper for Grammar {}

impl IGrammarRegistry for Grammar {
    fn get_external_grammar(
        &self,
        _scope_name: String,
        _repository: IRawRepository,
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

pub fn to_grammar_for_test(grammar_path: &str) -> Grammar {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(grammar_path);

    let mut file = File::open(path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let g: IRawGrammar = serde_json::from_str(&data).unwrap();
    Grammar::new(g)
}

pub fn to_grammar_with_code(grammar_path: &str, code: &str) -> Grammar {
    let mut grammar = to_grammar_for_test(grammar_path);
    let c_code = String::from(code);
    let mut rule_stack = Some(StackElement::null());
    for line in c_code.lines() {
        let result = grammar.tokenize_line(String::from(line), &mut rule_stack);
        rule_stack = *result.rule_stack;
        for token in result.tokens {
            let start = token.start_index.clone() as usize;
            let end = token.end_index.clone() as usize;
            let new_line: String = String::from(line)
                .chars()
                .skip(start)
                .take(end - start)
                .collect();
            let token_str: String = token.scopes.join(", ");
            println!(
                " - token from {:?} to {:?} ({:?}) with scopes {:?}",
                token.start_index, token.end_index, new_line, token_str
            )
        }
    }

    grammar
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;

    use crate::grammar::grammar::{to_grammar_for_test, to_grammar_with_code};
    use crate::grammar::{Grammar, StackElement};
    use crate::rule::abstract_rule::RuleEnum;
    use crate::rule::IRuleRegistry;
    use crate::grammar::line_tokens::IToken;

    #[test]
    fn should_build_grammar_json() {
        let code = "
#include <stdio.h>
int main() {
printf(\"Hello, World!\");
return 0;
}
";
        let grammar = to_grammar_with_code("test-cases/first-mate/fixtures/c.json", code);
        let first_rule = grammar.rule_id2desc.get(&1).unwrap();
        assert_eq!(28, first_rule.clone().patterns_length());
        debug_output(&grammar, String::from("program.json"));
    }

    #[test]
    fn should_identify_c_include() {
        let code = "#include <stdio.h>";
        let mut grammar = to_grammar_for_test("test-cases/first-mate/fixtures/c.json");
        let mut rule_stack = Some(StackElement::null());
        let result = grammar.tokenize_line(String::from(code), &mut rule_stack);

        assert_eq!(6, result.tokens.len());
        assert_eq!(0, result.tokens[0].start_index);
        assert_eq!(1, result.tokens[1].start_index);
        assert_eq!(8, result.tokens[2].start_index);
        assert_eq!(9, result.tokens[3].start_index);
        assert_eq!(10, result.tokens[4].start_index);
        assert_eq!(17, result.tokens[5].start_index);
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
        let grammar = to_grammar_with_code("test-cases/first-mate/fixtures/json.json", code);
        assert_eq!(grammar.rule_id2desc.len(), 22);
        debug_output(&grammar, String::from("program.json"));
    }

    #[test]
    fn should_build_html_grammar() {
        let code = "<html><html>";
        let grammar = to_grammar_with_code("test-cases/first-mate/fixtures/html.json", code);
        assert_eq!(grammar.rule_id2desc.len(), 101);
        debug_output(&grammar, String::from("program.json"));
    }

    #[test]
    fn should_build_correct_end_rule_id_for_makefile() {
        let code = "CC=gcc
CFLAGS=-I.
DEPS = hellomake.h
OBJ = hellomake.o hellofunc.o
";
        let mut grammar =
            to_grammar_with_code("test-cases/first-mate/fixtures/makefile.json", code);
        let mut end_rule_count = 0;
        for (_x, rule) in grammar.rule_id2desc.clone() {
            let rule_instance = rule.get_rule_instance();
            if let RuleEnum::BeginEndRule(rule) = rule_instance {
                assert_eq!(rule._end.rule_id, -1);
                end_rule_count = end_rule_count + 1;
            }
        }
        assert_eq!(grammar.get_rule(1).patterns_length(), 4);
        assert_eq!(end_rule_count, 24);
        debug_output(&grammar, String::from("program.json"));
    }

    #[test]
    fn should_build_makefile_grammar() {
        let code = "CC=gcc
CFLAGS=-I.
DEPS = hellomake.h
OBJ = hellomake.o hellofunc.o

%.o: %.c $(DEPS)
\t$(CC) -c -o $@ $< $(CFLAGS)

hellomake: $(OBJ)
\t$(CC) -o $@ $^ $(CFLAGS)";
        let mut grammar =
            to_grammar_with_code("test-cases/first-mate/fixtures/makefile.json", code);
        assert_eq!(grammar.rule_id2desc.len(), 82);
        assert_eq!(grammar.get_rule(1).patterns_length(), 4);

        let tokens = get_all_tokens("test-cases/first-mate/fixtures/makefile.json", code.clone());
        assert_eq!(10, tokens.len());
        let x: Vec<String> = tokens.iter().map(|token| token.len().to_string()).collect();
        assert_eq!(String::from("3,3,4,4,1,9,14,1,4,14"), x.join(","));
    }


    pub fn get_all_tokens(grammar_path: &str, code: &str) -> Vec<Vec<IToken>> {
        let mut grammar = to_grammar_for_test(grammar_path);
        let c_code = String::from(code);
        let mut rule_stack = Some(StackElement::null());
        let mut all_tokens: Vec<Vec<IToken>> = vec![];

        for line in c_code.lines() {
            let result = grammar.tokenize_line(String::from(line), &mut rule_stack);
            println!("{:?}", rule_stack.unwrap().rule_id.clone());
            rule_stack = *result.rule_stack;
            all_tokens.push(result.tokens);
        }

        all_tokens
    }

    #[test]
    fn should_resolve_make_file_error_issues() {
        let mut grammar = to_grammar_for_test("test-cases/first-mate/fixtures/makefile.json");
        let result = grammar.tokenize_line(String::from("%.o: %.c $(DEPS)"), &mut None);
        let tokens = result.tokens.clone();
        assert_eq!(9, tokens.len());
        assert_eq!("Makefile,meta.scope.target.makefile,entity.name.function.target.makefile,constant.other.placeholder.makefile", tokens[0].scopes.join(","));
        assert_eq!(0, tokens[0].start_index);
        assert_eq!(1, tokens[1].start_index);
        assert_eq!(3, tokens[2].start_index);
        assert_eq!(4, tokens[3].start_index);
        assert_eq!(5, tokens[4].start_index);
        assert_eq!(6, tokens[5].start_index);
        assert_eq!(9, tokens[6].start_index);
        assert_eq!(11, tokens[7].start_index);
        assert_eq!(15, tokens[8].start_index);
        debug_output(&grammar, String::from("program.json"));
    }

    #[test]
    fn should_resolve_make_file_error_issues2() {
        let mut grammar = to_grammar_for_test("test-cases/first-mate/fixtures/makefile.json");

        let mut rule_stack = Some(StackElement::null());
        let result = grammar.tokenize_line(String::from("hellomake: $(OBJ)"), &mut rule_stack);
        assert_eq!(6, result.tokens.len());

        rule_stack = *result.rule_stack;
        let result2 =
            grammar.tokenize_line(String::from("\t$(CC) -o $@ $^ $(CFLAGS)"), &mut rule_stack);
        assert_eq!(14, result2.tokens.len());
    }

    #[test]
    fn should_success_token_for_short_code() {
        let code = "hellomake: $(OBJ)
\t$(CC) -o $@ $^ $(CFLAGS)";
        let tokens = get_all_tokens("test-cases/first-mate/fixtures/makefile.json", code.clone());
        assert_eq!(2, tokens.len());
        let x: Vec<String> = tokens.iter().map(|token| token.len().to_string()).collect();
        assert_eq!(String::from("6,14"), x.join(","));
    }
}
