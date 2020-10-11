extern crate serde;

use walkdir::{WalkDir};
use std::path::{PathBuf};
use scie_infra::finder::Finder;
use scie_model::{JsonPackage, TMGrammar};
use std::collections::HashMap;

pub fn walk_dir(path: String) -> Vec<PathBuf> {
    let mut packages = vec![];
    let walk_dir = WalkDir::new(path);

    let filtered_entries = walk_dir.max_depth(2).into_iter();
    for entry in filtered_entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.display().to_string().ends_with("package.json") {
            packages.push(path.to_path_buf());
        }
    }

    packages
}

pub struct ExtEntry {
    pub name: String,
    pub path: String,
}

pub struct LangExtMap {
    pub ext_map: HashMap<String, ExtEntry>,
    pub grammar_map: HashMap<String, TMGrammar>,
}

impl LangExtMap {
    pub fn new() -> Self {
        LangExtMap {
            ext_map: Default::default(),
            grammar_map: Default::default(),
        }
    }
}


fn main() {}

fn build_languages_map(ext_path: PathBuf) -> LangExtMap {
    let package_files = walk_dir(ext_path.to_str().unwrap().to_string());
    let mut lang_ext_map = LangExtMap::new();

    for path in package_files {
        let package = Finder::read_code(&path);
        let pkg: JsonPackage = match serde_json::from_str(&package) {
            Ok(x) => x,
            Err(err) => {
                println!("{:?}, {:?}", &path, err);
                panic!(err)
            }
        };

        if let Some(grammars) = pkg.contributes.grammars {
            for grammar in grammars {
                if let Some(lang) = grammar.language.clone() {
                    lang_ext_map.grammar_map.insert(lang, grammar);
                }
            }
        }

        if let Some(exts) = pkg.contributes.languages {
            for lang_ext in exts {
                let lang_id = lang_ext.id;
                if let Some(extensions) = lang_ext.extensions {
                    for ext in extensions {
                        lang_ext_map.ext_map.insert(ext, ExtEntry {
                            name: lang_id.clone(),
                            path: path.parent().unwrap().as_os_str().to_str().unwrap().to_string(),
                        });
                    }
                }
            }
        }
    }

    lang_ext_map
}

#[cfg(test)]
mod tests {
    use std::path::{PathBuf};
    use crate::build_languages_map;

    #[test]
    fn should_get_css_scope_name() {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf();
        let ext_path = root_dir.join("extensions");

        let languages_map = build_languages_map(ext_path);
        assert_eq!("css", languages_map.ext_map[".css"].name);
        assert!(languages_map.ext_map[".css"].path.ends_with("css"));

        assert_eq!("source.css", languages_map.grammar_map["css"].scope_name);
        assert_eq!("./syntaxes/css.tmLanguage.json", languages_map.grammar_map["css"].path);
    }

    #[test]
    fn should_build_css_raw_grammar_path() {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf();
        let ext_path = root_dir.join("extensions");

        let languages_map = build_languages_map(ext_path.clone());
        let css_path = languages_map.grammar_map["css"].path.clone();

        let path = ext_path
            .join(languages_map.ext_map[".css"].path.clone())
            .join(css_path);
        // let string = Finder::read_code(&path.to_path_buf());
        assert!(path.exists())
    }
}
