use crate::inter::IRawRule;
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct InjectionMap {
    #[serde(flatten)]
    pub map: BTreeMap<String, IRawRule>,
}
