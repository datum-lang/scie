use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CodeFile {
    pub path: String,
    pub elements: Vec<Element>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Element {
    #[serde(alias = "lineNum")]
    pub line_num: i32,
    #[serde(alias = "start")]
    pub start_index: i32,
    #[serde(alias = "end")]
    pub end_index: i32,
    pub value: String,
    pub scopes: Vec<ElementScope>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ElementScope {
    #[serde(alias = "scopeName")]
    pub name: String,
    pub index: i32,
}
