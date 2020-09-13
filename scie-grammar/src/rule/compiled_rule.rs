use scie_scanner::scanner::scie_scanner::ScieScanner;

#[derive(Clone, Debug, Serialize)]
pub struct CompiledRule {
    pub debug_reg_exps: Vec<String>,
    pub rules: Vec<i32>,
    pub scanner: Box<ScieScanner>,
}

impl CompiledRule {
    pub fn new(debug_reg_exps: Vec<String>, rules: Vec<i32>) -> Self {
        let scanner = ScieScanner::new(debug_reg_exps.clone());
        CompiledRule {
            debug_reg_exps,
            rules,
            scanner: Box::new(scanner),
        }
    }

    pub fn dispose(&self) {
        unsafe {
            self.scanner.dispose();
        }
    }
}
