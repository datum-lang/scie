pub mod location;
pub mod raw_grammar;
pub mod raw_repository;
pub mod raw_rule;
pub mod raw_captures;
pub mod injections;

pub use self::location::ILocatable;
pub use self::location::ILocation;
pub use self::raw_grammar::IRawGrammar;
pub use self::raw_repository::IRawRepository;
pub use self::raw_repository::IRawRepositoryMap;
pub use self::raw_captures::IRawCapturesMap;
pub use self::raw_captures::IRawCaptures;
pub use self::raw_rule::IRawRule;
pub use self::injections::InjectionMap;

#[cfg(test)]
mod tests {
    use crate::inter::{IRawCaptures, IRawGrammar, IRawRule, InjectionMap};
    use serde::{Deserialize, Serialize};
    use std::fs;
    use std::fs::File;
    use std::io::Read;
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

    #[test]
    fn should_convert_patterns() {
        let data = r#"
        {
        	"patterns": [
                {
                    "captures": {
                        "1": {
                            "name": "keyword.other.package.java"
                        },
                        "2": {
                            "name": "storage.modifier.package.java"
                        },
                        "3": {
                            "name": "punctuation.terminator.java"
                        }
                    },
                    "match": "^\\s*(package)\\b(?:\\s*([^ ;$]+)\\s*(;)?)?",
                    "name": "meta.package.java"
                }
            ]
        }"#;

        let p: IRawGrammar = serde_json::from_str(data).unwrap();
        let pattern = p.patterns;
        assert_eq!(
            "meta.package.java",
            String::from(pattern[0].clone().name.unwrap())
        );
        assert_eq!(
            "^\\s*(package)\\b(?:\\s*([^ ;$]+)\\s*(;)?)?",
            String::from(pattern[0].clone().match_s.unwrap())
        );
        assert_eq!(
            3,
            pattern[0].clone().captures.unwrap().map.capture_map.len()
        );
    }

    #[test]
    fn should_convert_json_file() {
        let path = "test-cases/first-mate/fixtures/c.json";
        let mut file = File::open(path).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        let p: IRawRule = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn should_convert_json_files_list() {
        let path = Path::new("test-cases/first-mate/fixtures/");
        for entry in fs::read_dir(path).expect("Unable to list") {
            let entry = entry.expect("unable to get entry");

            let mut file = File::open(entry.path()).unwrap();
            let mut data = String::new();
            file.read_to_string(&mut data).unwrap();
            let p: IRawGrammar = match serde_json::from_str(&data) {
                Ok(x) => x,
                Err(err) => {
                    println!("{:?}", file);
                    println!("{:?}", err);
                    IRawGrammar::new()
                }
            };
            assert_eq!(true, p.scope_name.unwrap().len() > 0);
        }
    }

    #[test]
    fn should_read_java_repository() {
        let path = Path::new("test-cases/first-mate/fixtures/java.json");

        let mut file = File::open(path).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let p: IRawGrammar = match serde_json::from_str(&data) {
            Ok(x) => x,
            Err(err) => IRawGrammar::new(),
        };
        assert_eq!(25, p.repository.unwrap().map.name_map.len());
    }
}
