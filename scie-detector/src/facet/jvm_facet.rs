#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct JvmFacet {
    pub is_gradle: bool,
    pub is_maven: bool,

    pub has_java: bool,
    pub has_groovy: bool,
    pub has_kotlin: bool,
    pub has_scala: bool,
}
