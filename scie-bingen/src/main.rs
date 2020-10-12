#![allow(dead_code)]

#[macro_use]
extern crate serde_derive;

extern crate serde;

use scie_infra::finder::Finder;
use scie_model::{JsonPackage, TMGrammar};
use std::collections::HashMap;
use std::path::PathBuf;
use walkdir::WalkDir;
use std::fs::File;
use std::io::Write;

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtEntry {
    pub name: String,
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

        if pkg.contributes.languages.is_none() {
            continue;
        }

        for lang_ext in pkg.contributes.languages.unwrap() {
            if lang_ext.extensions.is_none() {
                continue;
            }

            for ext in lang_ext.extensions.unwrap() {
                let mut path = path.parent().unwrap().display().to_string();
                path = path.replace(".//", "");

                let ext_entry = ExtEntry {
                    name: lang_ext.id.clone(),
                    path,
                };
                lang_ext_map.ext_map.insert(ext, ext_entry);
            }
        }
    }

    lang_ext_map
}

fn write_to_file(map: &LangExtMap, path: &str) {
    let json_str = serde_json::to_string_pretty(&map).unwrap();
    let bytes = json_str.as_bytes();

    let mut file = File::create(path).unwrap();
    match file.write_all(bytes) {
        Ok(_) => {}
        Err(_) => {}
    };
}

#[cfg(test)]
mod tests {
    use crate::{build_languages_map, write_to_file};
    use std::path::PathBuf;

    #[test]
    fn should_get_css_scope_name() {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();
        let ext_path = root_dir.join("extensions");

        let languages_map = build_languages_map(ext_path);
        assert_eq!("css", languages_map.ext_map[".css"].name);
        assert!(languages_map.ext_map[".css"].path.ends_with("css"));

        assert_eq!("source.css", languages_map.grammar_map["css"].scope_name);
        assert_eq!(
            "./syntaxes/css.tmLanguage.json",
            languages_map.grammar_map["css"].path
        );
    }

    #[test]
    fn should_build_css_raw_grammar_path() {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();
        let ext_path = root_dir.join("extensions");

        let languages_map = build_languages_map(ext_path.clone());

        let css_path = languages_map.grammar_map["css"].path.clone();
        let parent_path = languages_map.ext_map[".css"].path.clone();

        let path = ext_path
            .join(parent_path)
            .join(css_path);

        write_to_file(&languages_map, "map.json");
        assert!(path.exists())
    }
}
