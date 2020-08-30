use crate::inter::{IRawCaptures, ILocation, IRawRepository};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct IRawRule {
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ILocation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(alias = "contentName")]
    pub content_name: Option<String>,

    #[serde(alias = "match")]
    pub match_s: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captures: Option<Box<IRawCaptures>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin: Option<String>,

    #[serde(alias = "beginCaptures", skip_serializing_if = "Option::is_none")]
    pub begin_captures: Option<Box<IRawCaptures>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    #[serde(alias = "endCaptures", skip_serializing_if = "Option::is_none")]
    pub end_captures: Option<Box<IRawCaptures>>,

    #[serde(alias = "while", skip_serializing_if = "Option::is_none")]
    pub while_s: Option<String>,
    #[serde(alias = "whileCaptures", skip_serializing_if = "Option::is_none")]
    pub while_captures: Option<Box<IRawCaptures>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub patterns: Option<Vec<IRawRule>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<IRawRepository>,

    #[serde(alias = "applyEndPatternLast", skip_serializing_if = "Option::is_none")]
    pub apply_end_pattern_last: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub information_for_contributors: Option<Vec<String>>,
}

impl IRawRule {
    pub fn new() -> Self {
        IRawRule {
            id: None,
            location: None,
            include: None,
            name: None,
            content_name: None,
            match_s: None,
            captures: None,
            begin: None,
            begin_captures: None,
            end: None,
            end_captures: None,
            while_s: None,
            while_captures: None,
            patterns: None,
            repository: None,
            information_for_contributors: None,
            apply_end_pattern_last: None,
        }
    }
}
