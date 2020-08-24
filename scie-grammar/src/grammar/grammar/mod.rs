pub mod scope_list_element;
pub mod scope_metadata;
pub mod stack_element;

use crate::grammar::grammar::stack_element::StackElement;
use crate::inter::{ILocation, IRawGrammar, IRawRepository, IRawRule, IRawRepositoryMap};
use crate::rule::{
    AbstractRule, IGrammarRegistry, IRuleFactoryHelper, IRuleRegistry, Rule,
    RuleFactory,
};
use onig::*;
use std::borrow::Borrow;
use std::collections::HashMap;

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
pub struct TokenTypeMatcher {}

#[derive(Debug, Clone)]
pub struct Grammar {
    root_id: i32,
    grammar: IRawGrammar,
    pub last_rule_id: i32,
    pub rule_id2desc: HashMap<i32, Box<dyn AbstractRule>>,
    pub _token_type_matchers: Vec<TokenTypeMatcher>,
}

pub fn init_grammar(grammar: IRawGrammar, base: Option<IRawRule>) -> IRawGrammar {
    let mut _grammar = grammar.clone();

    // _grammar.repository = grammar.repository.clone();

    let mut new_based: IRawRule = IRawRule::new();
    new_based.location = grammar.clone().repository.unwrap().clone().location;
    new_based.patterns = Some(grammar.clone().patterns.clone());
    new_based.name = grammar.clone().name;

    let mut repository_map = IRawRepositoryMap::new();
    repository_map.base_s = Some(new_based.clone());
    repository_map.self_s = Some(new_based.clone());
    repository_map.name_map = grammar.clone().repository.unwrap().clone().map.name_map.clone();

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
            rule_id2desc: Default::default(),
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
        prev_state: Option<StackElement>,
        emit_binary_tokens: bool,
    ) {
        if self.root_id == -1 {
            let repository = self.grammar.repository.clone().unwrap();
            let based = repository.clone().map.base_s.unwrap();
            self.root_id =
                RuleFactory::get_compiled_rule_id(based.clone(), self, repository.clone());
        }

        let mut is_first_line: bool = false;
        if let None = prev_state {
            is_first_line = true
        } else {
        }

        let lineText = format!("{:?}\n", line_text);
        let onigLineText = self.create_onig_string(lineText);
        self.tokenize_string(onigLineText.parse().unwrap(), is_first_line, 0, true)
    }

    pub fn tokenize_string(
        &mut self,
        line_text: String,
        is_first_line: bool,
        line_pos: i32,
        check_while_conditions: bool,
    ) {
        let line_length = line_text.len();

        let mut stop = false;
        let mut anchorPosition = -1;

        if check_while_conditions {
            self.check_while_conditions(line_text.clone(), is_first_line, line_pos)
        }

        self.match_rule_or_injections(line_text, is_first_line, line_pos, anchorPosition);
    }

    pub fn check_while_conditions(
        &mut self,
        line_text: String,
        is_first_line: bool,
        line_pos: i32,
    ) {
    }

    pub fn match_rule_or_injections(
        &mut self,
        line_text: String,
        is_first_line: bool,
        line_pos: i32,
        anchor_position: i32,
    ) {
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
        self.last_rule_id
    }

    fn get_rule(&self, pattern_id: i32) -> Rule {
        Rule::new(ILocation::new(), pattern_id, None, None)
    }

    fn register_rule(&mut self, result: Box<dyn AbstractRule>) -> Box<dyn AbstractRule> {
        self.rule_id2desc
            .insert(self.last_rule_id.clone(), result.clone());
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::grammar::grammar::Grammar;
    use crate::inter::IRawGrammar;
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    #[test]
    fn should_enable_run_grammar() {
        let path = Path::new("test-cases/first-mate/fixtures/java.json");
        let mut file = File::open(path).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        let g: IRawGrammar = serde_json::from_str(&data).unwrap();

        let mut grammar = Grammar::new(g);
        let c_code = String::from(
            "
#include <stdio.h>
int main() {
   printf(\"Hello, World!\");
   return 0;
}
",
        );
        for line in c_code.lines() {
            grammar.tokenize_line(String::from(line), None)
        }
    }
}
