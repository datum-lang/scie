use crate::inter::ILocation;

#[derive(Clone, Debug, Serialize)]
pub struct Rule {
    pub _type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _location: Option<ILocation>,
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _content_name: Option<String>,
}

impl Rule {
    pub fn new(
        location: ILocation,
        id: i32,
        name: Option<String>,
        content_name: Option<String>,
    ) -> Self {
        Rule {
            _type: "".to_string(),
            _location: Some(location),
            id,
            _name: name,
            _content_name: content_name,
        }
    }
}
