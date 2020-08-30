use crate::inter::IRawRule;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct InjectionMap {
    #[serde(flatten)]
    pub map: HashMap<String, IRawRule>,
}
