use crate::raw_language::RawLanguageExt;
use crate::tm_grammar::TMGrammar;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Contribute {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<RawLanguageExt>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grammars: Option<Vec<TMGrammar>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(alias = "breakPoints")]
    pub break_points: Option<Vec<BreakPoint>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snippets: Option<Vec<ContribSnippet>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BreakPoint {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContribSnippet {}
