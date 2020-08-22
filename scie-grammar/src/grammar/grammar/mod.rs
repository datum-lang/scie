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
    pub rule_stack: Box<StackElement>
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