#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TokenElement {
    #[serde(alias = "line")]
    pub line_num: i32,
    #[serde(alias = "start")]
    pub start_index: i32,
    #[serde(alias = "end")]
    pub end_index: i32,
    pub value: String,
    pub scopes: Vec<String>,
}

impl TokenElement {
    pub fn new(
        line_num: i32,
        start_index: i32,
        end_index: i32,
        value: String,
        scopes: Vec<String>,
    ) -> TokenElement {
        TokenElement {
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
