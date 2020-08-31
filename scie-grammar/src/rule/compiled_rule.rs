#[derive(Clone, Debug, Serialize)]
pub struct CompiledRule {
    pub debug_reg_exps: Vec<String>,
    pub rules: Vec<i32>,
}

impl CompiledRule {
    pub fn new(rules: Vec<i32>) -> Self {
        CompiledRule {
            debug_reg_exps: vec![],
            rules,
        }
    }
}
