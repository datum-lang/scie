
pub type Pointer = i32;

pub struct OnigScanner {
    pub _ptr: Pointer
}

impl OnigScanner {
    pub fn new(pattens: Vec<&str>) -> Self {
        let _str_ptrs_arr: Vec<Pointer> = vec![];
        let _str_len_arr: Vec<i32> = vec![];
        for _x in pattens {

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
