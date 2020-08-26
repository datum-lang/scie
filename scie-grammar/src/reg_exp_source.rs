#[derive(Clone, Debug, Serialize)]
pub struct RegExpSourceList {}

#[derive(Clone, Debug, Serialize)]
pub struct RegExpSource {
    pub source: String,
    pub rule_id: i32,
}

impl RegExpSource {
    pub fn new(reg_exp_source: String, rule_id: i32) -> RegExpSource {
        RegExpSource {
            source: reg_exp_source,
            rule_id,
        }
    }
}
