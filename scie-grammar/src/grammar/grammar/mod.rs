use crate::inter::IRawGrammar;
use crate::rule::RuleFactory;
use onig::*;

pub struct StackElement {}

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

pub struct Grammar {
    root_id: i32,
    grammar: IRawGrammar,
}

impl Grammar {
    pub fn new(grammar: IRawGrammar) -> Grammar {
        Grammar {
            grammar,
            root_id: -1,
        }
    }
    // todo: refactor to callback ??
    pub fn create_onig_scanner(&self, sources: String) -> Regex {
        // reg.scan(to_match, scan_callback)
        Regex::new(sources.as_str()).unwrap()
    }
    // todo: refactor to callback ??
    pub fn create_onig_string(&self, sources: String) -> Regex {
        // regex.captures("hello").unwrap().iter_pos().enumerate()
        Regex::new(sources.as_str()).unwrap()
    }

    fn tokenize(
        &self,
        line_text: String,
        prev_state: Option<StackElement>,
        emit_binary_tokens: bool,
    ) {
        if self.root_id == -1 {
            RuleFactory::get_compiled_rule_id(self.grammar.repository.clone())
        }
    }

    pub fn tokenize_line(&self, line_text: String, prev_state: Option<StackElement>) {
        self.tokenize(line_text, prev_state, false)
    }

    pub fn tokenize_line2(&self, line_text: String, prev_state: Option<StackElement>) {}
}
