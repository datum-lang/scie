use crate::ext_file::ExtFile;
use scie_infra::finder::Finder;
use scie_model::{JsonPackage, TMGrammar};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ExtEntry {
    pub name: String,
    pub path: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct LangExtGen {
    pub ext_map: HashMap<String, ExtEntry>,
    pub grammar_map: HashMap<String, TMGrammar>,
}

impl Default for LangExtGen {
    fn default() -> Self {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();
        let ext_path = root_dir.join("extensions");

        LangExtGen::from_path(ext_path)
    }
}

impl LangExtGen {
    pub fn new() -> Self {
        LangExtGen {
            ext_map: Default::default(),
            grammar_map: Default::default(),
        }
    }

    pub fn to_json_file(&self, path: &str) {
        let json_str = serde_json::to_string_pretty(&self).unwrap();
        let bytes = json_str.as_bytes();

        let mut file = File::create(path).unwrap();
        match file.write_all(bytes) {
            Ok(_) => {}
            Err(_) => {}
        };
    }

    pub fn to_bin_file(&self, path: &str) {
        let encoded: Vec<u8> = bincode::serialize(&self).unwrap();
        let mut file = File::create(path).unwrap();
        match file.write_all(&*encoded) {
            Ok(_) => {}
            Err(_) => {}
        };
    }

    pub fn from_path(ext_path: PathBuf) -> LangExtGen {
        let package_files = ExtFile::walk_dir(ext_path.to_str().unwrap().to_string());
        let mut lang_ext_map = LangExtGen::new();

        for path in package_files {
            let package = Finder::read_code(&path).unwrap();
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
}

#[cfg(test)]
mod tests {
    use crate::language_gen::{ExtEntry, LangExtGen};
    use std::collections::HashMap;
    use std::path::PathBuf;

    #[test]
    fn should_get_css_scope_name() {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();
        let ext_path = root_dir.join("extensions");

        let languages_map = LangExtGen::from_path(ext_path);
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

        let languages_map = LangExtGen::from_path(ext_path.clone());

        let css_path = languages_map.grammar_map["css"].path.clone();
        let parent_path = languages_map.ext_map[".css"].path.clone();

        let path = ext_path.join(parent_path).join(css_path);

        languages_map.to_json_file("lang_map.json");
        languages_map.to_bin_file("lang_map.bin");
        assert!(path.exists())
    }

    #[test]
    fn should_build_bin_data_for_hashmap() {
        let mut map: HashMap<String, ExtEntry> = Default::default();
        map.insert(
            String::from(".css"),
            ExtEntry {
                name: "css".to_string(),
                path: "css".to_string(),
            },
        );

        assert!(bincode::serialize(&map).is_ok());
    }
}
