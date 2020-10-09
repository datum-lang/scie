pub struct Workspace {
    pub projects: Vec<Project>,
}

pub struct Project {
    pub modules: Vec<Module>
}

pub struct Module {
    pub files: Vec<Package>,
    pub artifacts: Vec<Artifact>,
}

pub struct Artifact {
    pub name: String,
    pub out: String,
}

pub struct Package {
    pub files: Vec<File>,
}

pub struct File {
    pub elements: Vec<Element>
}

pub struct Element {
    pub scopes: Vec<ElementScope>,
}

pub struct ElementScope {
    pub start_index: i32,
    pub end_index: i32,
    pub scope_name: String,
    pub value: String,
}
