
pub type Pointer = i32;

pub struct OnigScanner {
    pub _ptr: Pointer
}

impl OnigScanner {
    pub fn new(pattens: Vec<&str>) -> Self {
        let strPtrsArr: Vec<Pointer> = vec![];
        let strLenArr: Vec<i32> = vec![];
        for x in pattens {

        }
        OnigScanner { _ptr: 0 }
    }
}


#[cfg(test)]
mod tests {
    use crate::scanner::onig_scanner::OnigScanner;

    #[test]
    fn it_show_works_works() {
        OnigScanner::new(vec!["^"]);
        assert!(true)
    }
}
