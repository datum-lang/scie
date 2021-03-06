use crate::inter::{ILocation, IRawRule};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct IRawRepository {
    #[serde(flatten)]
    pub map: Box<IRawRepositoryMap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ILocation>,
}

impl IRawRepository {
    pub fn new() -> Self {
        IRawRepository {
            map: Box::new(IRawRepositoryMap::new()),
            location: None,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct IRawRepositoryMap {
    #[serde(flatten)]
    pub name_map: BTreeMap<String, Box<IRawRule>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_s: Option<Box<IRawRule>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_s: Option<Box<IRawRule>>,
}

impl IRawRepositoryMap {
    pub fn new() -> Self {
        IRawRepositoryMap {
            name_map: Default::default(),
            self_s: None,
            base_s: None,
        }
    }
}
