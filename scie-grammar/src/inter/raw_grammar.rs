use crate::inter::{ILocatable, IRawRepository, IRawRule, InjectionMap};

// #[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct IRawGrammar {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<IRawRepository>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ILocatable>,

    #[serde(alias = "scopeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_name: Option<String>,

    pub patterns: Vec<IRawRule>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub injections: Option<InjectionMap>,

    #[serde(alias = "injectionSelector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub injection_selector: Option<String>,

    #[serde(alias = "fileTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_types: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(alias = "firstLineMatch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_line_match: Option<String>,

    // not in list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    // pub foldingStartMarker: Option<String>,
    // pub foldingStopMarker: Option<String>,
    // pub keyEquivalent: Option<String>,
    // pub hideFromUser: Option<bool>,

    // #[serde(skip_serializing)]
    // ignored_field: serde::de::IgnoredAny,
}

impl IRawGrammar {
    pub fn new() -> IRawGrammar {
        IRawGrammar {
            location: None,
            repository: None,
            scope_name: Some("".to_string()),
            patterns: vec![],
            injections: None,
            injection_selector: None,
            file_types: None,
            name: None,
            first_line_match: None,

            comment: None,
            // foldingStartMarker: None,
            // foldingStopMarker: None,
            // keyEquivalent: None,
            // hideFromUser: None,
        }
    }
}
