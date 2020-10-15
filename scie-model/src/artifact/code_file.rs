use crate::artifact::Element;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CodeFile {
    pub path: String,
    pub name: String,
    pub elements: Vec<Element>,
}

impl CodeFile {
    pub fn new(path: PathBuf) -> CodeFile {
        let name = path.file_name().unwrap().to_str().unwrap();
        CodeFile {
            path: path.display().to_string(),
            name: String::from(name),
            elements: vec![]
        }
    }
}

