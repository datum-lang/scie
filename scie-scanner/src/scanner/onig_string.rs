pub struct OnigString {
 pub utf16length: i32,
 pub utf8length: i32,
 pub utf16value: String,
 pub utf8value: Vec<u8>,
 pub utf16offset_to_utf8: Option<Vec<u32>>,
 pub utf8offset_to_utf16: Option<Vec<u32>>,

}

impl OnigString {
    pub fn new(str: String) -> Self {
        let utf16Length = str.len();
        

        OnigString {
            utf16length: 0,
            utf8length: 0,
            utf16value: "".to_string(),
            utf8value: vec![],
            utf16offset_to_utf8: None,
            utf8offset_to_utf16: None
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::scanner::onig_scanner::OnigScanner;
    use crate::scanner::onig_string::OnigString;

    #[test]
    fn it_show_works_works() {
        OnigString::new(String::from(""));
        assert!(true)
    }
}
