#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ILocation {
    pub filename: String,
    pub line: String,
    pub chart: String,
}

impl ILocation {
    pub fn new() -> Self {
        ILocation {
            filename: "".to_string(),
            line: "".to_string(),
            chart: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ILocatable {
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub textmate_location: Option<ILocation>,
}

impl ILocatable {
    pub fn new() -> Self {
        ILocatable {
            textmate_location: None,
        }
    }
}