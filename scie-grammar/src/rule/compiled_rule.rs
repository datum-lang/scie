use scie_scanner::scanner::scanner::Scanner;

#[derive(Clone, Debug, Serialize)]
pub struct CompiledRule {
    pub debug_reg_exps: Vec<String>,
    pub rules: Vec<i32>,
    pub scanner: Box<Scanner>,
}

impl CompiledRule {
    pub fn new(debug_reg_exps: Vec<String>, rules: Vec<i32>) -> Self {
        // println!("{:?}", rules);
        let scanner = Scanner::new(debug_reg_exps.clone());
        CompiledRule {
            debug_reg_exps,
            rules,
            scanner: Box::new(scanner),
        }
    }
}
