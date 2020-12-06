#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Element {
    #[serde(alias = "line")]
    pub line_num: i32,
    #[serde(alias = "start")]
    pub start_index: i32,
    #[serde(alias = "end")]
    pub end_index: i32,
    pub value: String,
    pub scopes: Vec<String>,
}

impl Element {
    pub fn new(
        line_num: i32,
        start_index: i32,
        end_index: i32,
        value: String,
        scopes: Vec<String>,
    ) -> Element {
        Element {
            line_num,
            start_index,
            end_index,
            value,
            scopes,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ElementScope {
    pub name: String,
    pub index: i32,
}
