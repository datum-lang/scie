use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Workspace {
    #[serde(alias = "workspaceName")]
    pub name: String,
    pub projects: Vec<Project>,
}

#[derive(Serialize, Deserialize)]
pub struct Project {
    #[serde(alias = "projectName")]
    pub name: String,
    pub modules: Vec<Module>,
}

#[derive(Serialize, Deserialize)]
pub struct Module {
    #[serde(alias = "moduleName")]
    pub name: String,
    pub packages: Vec<Package>,
    pub artifacts: Vec<Artifact>,
}

#[derive(Serialize, Deserialize)]
pub struct Artifact {
    #[serde(alias = "artifactName")]
    pub name: String,
    pub out: String,
}

#[derive(Serialize, Deserialize)]
pub struct Package {
    #[serde(alias = "packageName")]
    pub name: String,
    pub files: Vec<CodeFile>,
}

#[derive(Serialize, Deserialize)]
pub struct CodeFile {
    pub elements: Vec<Element>,
    pub scopes: Vec<ElementScope>
}

#[derive(Serialize, Deserialize)]
pub struct Element {
    #[serde(alias = "lineNum")]
    pub line_num: i32,
    #[serde(alias = "start")]
    pub start_index: i32,
    #[serde(alias = "end")]
    pub end_index: i32,
    pub value: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ElementScope {
    #[serde(alias = "scopeName")]
    pub name: String,
    pub index: i32
}
