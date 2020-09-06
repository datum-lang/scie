use crate::grammar::{ScopeListElement, StackElement};

#[derive(Debug, Clone)]
pub struct TokenTypeMatcher {}

#[derive(Debug, Clone)]
pub struct IToken {
    pub start_index: i32,
    pub end_index: i32,
    pub scopes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct LineTokens {
    pub emit_binary_tokens: bool,
    pub _line_text: String,
    pub _tokens: Vec<IToken>,
    pub _binary_tokens: Vec<IToken>,
    pub _last_token_end_index: i32,
    pub _token_type_overrides: Vec<TokenTypeMatcher>,
}

impl LineTokens {
    pub fn new(
        emit_binary_tokens: bool,
        _line_text: String,
        _token_type_overrides: Vec<TokenTypeMatcher>,
    ) -> Self {
        LineTokens {
            emit_binary_tokens,
            _line_text,
            _tokens: vec![],
            _binary_tokens: vec![],
            _last_token_end_index: 0,
            _token_type_overrides,
        }
    }

    pub fn produce(&mut self, stack: &mut StackElement, end_index: i32) {
        self.produce_from_scopes(&mut stack.content_name_scopes_list, end_index)
    }

    pub fn produce_from_scopes(&mut self, scopes_list: &mut ScopeListElement, end_index: i32) {
        if self._last_token_end_index >= end_index {
            return;
        }
        //
        // if self.emit_binary_tokens {
        //     let meta_data = scopes_list.metadata;
        // }

        let scopes = scopes_list.generate_scopes();
        self._tokens.push(IToken {
            start_index: 0,
            end_index,
            scopes,
        });
        self._last_token_end_index = end_index
    }
}
