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

impl Element {
    pub fn new(line_num: i32, start_index: i32, end_index: i32, value: String) -> Element {
        Element {
            line_num,
            start_index,
            end_index,
            value,
            scopes: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ElementScope {
    #[serde(alias = "scopeName")]
    pub name: String,
    pub index: i32,
}
