use serde::{Deserialize, Serialize};

pub mod code_element;
pub mod code_file;

pub use code_element::TokenElement;
pub use code_file::CodeFile;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Workspace {
    #[serde(alias = "workspaceName")]
    pub name: String,
    pub projects: Vec<Project>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Project {
    #[serde(alias = "projectName")]
    pub name: String,
    pub modules: Vec<Module>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Module {
    #[serde(alias = "moduleName")]
    pub name: String,
    pub packages: Vec<Package>, // or dir ?
    pub artifacts: Vec<Artifact>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Artifact {
    #[serde(alias = "artifactName")]
    pub name: String,
    pub out: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Package {
    #[serde(alias = "packageName")]
    pub name: String,
    pub files: Vec<CodeFile>,
}
