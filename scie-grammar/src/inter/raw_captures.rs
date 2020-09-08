use crate::inter::{ILocatable, IRawRule};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct IRawCapturesMap {
    #[serde(flatten)]
    pub capture_map: HashMap<String, IRawRule>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct IRawCaptures {
    #[serde(flatten)]
    pub map: IRawCapturesMap,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ILocatable>,
}
