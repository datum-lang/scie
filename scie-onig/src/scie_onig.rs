use crate::scie_scanner::ScieScanner;

pub struct ScieOnig {

}

impl ScieOnig {
    pub fn create_onig_scanner(sources: Vec<String>) -> ScieScanner {

        ScieScanner::new()
    }
}

