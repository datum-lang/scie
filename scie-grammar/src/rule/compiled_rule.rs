pub struct CompiledRule {
    pub debug_reg_exps: Vec<String>,
    pub rules: Vec<i32>
}

impl CompiledRule {
    pub fn new(debug_reg_exps: Vec<String>, rules: Vec<i32>) -> Self {
        CompiledRule { debug_reg_exps, rules }
    }
}