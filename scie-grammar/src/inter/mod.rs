use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ILocation {
    pub filename: String,
    pub line: String,
    pub chart: String,
}

impl ILocation {
    pub fn new() -> Self {
        ILocation {
            filename: "".to_string(),
            line: "".to_string(),
            chart: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ILocatable {
    #[serde(flatten)]
    pub textmate_location: Option<ILocation>,
}

impl ILocatable {
    pub fn new() -> Self {
        ILocatable {
            textmate_location: None,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct IRawCapturesMap {
    #[serde(flatten)]
    capture_map: HashMap<String, IRawRule>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct IRawRepositoryMap {
    #[serde(flatten)]
    name_map: HashMap<String, IRawRule>,
    self_s: Option<IRawRule>,
    base_s: Option<IRawRule>,
}

impl IRawRepositoryMap {
    pub fn new() -> Self {
        IRawRepositoryMap {
            name_map: Default::default(),
            self_s: None,
            base_s: None,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct IRawRepository {
    #[serde(flatten)]
    pub map: Box<IRawRepositoryMap>,
    pub location: Option<ILocation>,
}

impl IRawRepository {
    pub fn new() -> Self {
        IRawRepository {
            map: Box::new(IRawRepositoryMap::new()),
            location: None,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct IRawCaptures {
    #[serde(flatten)]
    pub map: IRawCapturesMap,
    pub location: Option<ILocatable>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct IRawRule {
    pub id: Option<i32>,
    pub location: Option<ILocation>,

    pub include: Option<String>,
    pub name: Option<String>,

    #[serde(alias = "contentName")]
    pub content_name: Option<String>,

    #[serde(alias = "match")]
    pub match_s: Option<String>,
    pub captures: Option<Box<IRawCaptures>>,

    pub begin: Option<String>,
    #[serde(alias = "beginCaptures")]
    pub begin_captures: Option<Box<IRawCaptures>>,

    pub end: Option<String>,
    #[serde(alias = "endCaptures")]
    pub end_captures: Option<Box<IRawCaptures>>,

    #[serde(alias = "while")]
    pub while_s: Option<String>,
    #[serde(alias = "whileCaptures")]
    pub while_captures: Option<Box<IRawCaptures>>,

    pub patterns: Option<Vec<IRawRule>>,
    pub repository: Option<IRawRepository>,
    pub apply_end_pattern_last: Option<bool>,

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
            apply_end_pattern_last: None,
            information_for_contributors: None,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct InjectionMap {
    #[serde(flatten)]
    map: HashMap<String, IRawRule>,
}

pub struct IRawGrammar {
    pub location: ILocatable,
    pub repository: IRawRepository,
    pub scope_name: String,
    pub patterns: Vec<IRawRule>,
    pub injections: Option<InjectionMap>,
    pub injection_selector: Option<String>,
    pub file_types: Option<Vec<String>>,
    pub name: Option<String>,
    pub first_line_match: Option<String>,
}

impl IRawGrammar {
    pub fn new() -> IRawGrammar {
        IRawGrammar {
            location: ILocatable {
                textmate_location: None,
            },
            repository: IRawRepository::new(),
            scope_name: "".to_string(),
            patterns: vec![],
            injections: None,
            injection_selector: None,
            file_types: None,
            name: None,
            first_line_match: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::inter::{IRawCaptures, IRawRepository, IRawRule, InjectionMap};
    use serde::{Deserialize, Serialize};
    use std::fs;
    use std::path::Path;

    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct Captures {
        captures: Option<Box<IRawCaptures>>,
    }

    #[test]
    fn should_convert_captures() {
        let data = r#"
        {
            "captures": {
				"1": {
					"name": "punctuation.definition.item.text"
				}
			}
        }"#;

        let p: Captures = serde_json::from_str(data).unwrap();
        let name = p
            .captures
            .unwrap()
            .map
            .capture_map
            .get("1")
            .unwrap()
            .name
            .clone();
        assert_eq!("punctuation.definition.item.text", name.unwrap())
    }

    #[test]
    fn should_convert_injections() {
        let data = r#"
        "injections": {
            "R:text.html - comment.block": {
                "comment": "Use R: to ensure this matches after any other injections.",
                "patterns": [
                    {
                        "match": "<",
                        "name": "invalid.illegal.bad-angle-bracket.html"
                    }
                ]
            }
        }"#;

        let p: InjectionMap = serde_json::from_str(data).unwrap();
        let pattern = p
            .map
            .get("R:text.html - comment.block")
            .unwrap()
            .patterns
            .clone();
        assert_eq!(1, pattern.clone().unwrap().len());
        assert_eq!(
            "<",
            pattern
                .clone()
                .unwrap()
                .first()
                .unwrap()
                .match_s
                .clone()
                .unwrap()
        )
    }

    #[test]
    fn should_convert_rawrule() {
        let data = r#"
       {
			"begin": "'''",
			"beginCaptures": {
				"0": {
					"name": "punctuation.definition.string.begin.coffee"
				}
			},
			"end": "'''",
			"endCaptures": {
				"0": {
					"name": "punctuation.definition.string.end.coffee"
				}
			},
			"name": "string.quoted.single.heredoc.coffee",
			"patterns": [
				{
					"captures": {
						"1": {
							"name": "punctuation.definition.escape.backslash.coffee"
						}
					},
					"match": "(\\\\).",
					"name": "constant.character.escape.backslash.coffee"
				}
			]
		}"#;

        let p: IRawRule = serde_json::from_str(data).unwrap();
        let capture_map = p.end_captures.unwrap().map.capture_map;
        assert_eq!(
            "punctuation.definition.string.end.coffee",
            capture_map.get("0").unwrap().name.clone().unwrap()
        );
    }

    #[test]
    fn should_convert_repository() {
        let data = r#"
        {
            "repository": {
                "function_names": {
                    "patterns": [
                        {
                            "match": "(?x)\n\\b(isNaN|isFinite|eval|uneval|parseInt|parseFloat|decodeURI|\ndecodeURIComponent|encodeURI|encodeURIComponent|escape|unescape|\nrequire|set(Interval|Timeout)|clear(Interval|Timeout))\\b",
                            "name": "support.function.coffee"
                        },
                        {
                            "match": "[a-zA-Z_$][\\w$]*",
                            "name": "entity.name.function.coffee"
                        },
                        {
                            "match": "\\d[\\w$]*",
                            "name": "invalid.illegal.identifier.coffee"
                        }
                    ]
                }
            }
        }"#;

        let p: IRawRule = serde_json::from_str(data).unwrap();
        let repository_map = p.repository.unwrap().map.name_map.clone();
        let pattern_len = repository_map
            .get("function_names")
            .unwrap()
            .patterns
            .clone()
            .unwrap()
            .len();
        assert_eq!(3, pattern_len)
    }
    //
    // #[test]
    // fn should_convert_json_file() {
    //     let path = Path::new("../../../../extensions/json/syntaxes/JSON.tmLanguage.json");
    //     for entry in fs::read_dir(path).expect("Unable to list") {
    //         let entry = entry.expect("unable to get entry");
    //         println!("{}", entry.path().display());
    //     }
    // }
}
