use unicode_segmentation::UnicodeSegmentation;

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
        let utf16length = str.len();
        let utf8_str = str.graphemes(true).collect::<Vec<&str>>().clone();
        println!("{:?}", utf8_str);
        let utf8length = utf8_str.len();


        OnigString {
            utf16length: utf16length as i32,
            utf8length: utf8length as i32,
            utf16value: "".to_string(),
            utf8value: vec![],
            utf16offset_to_utf8: None,
            utf8offset_to_utf16: None
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::scanner::onig_string::OnigString;

    #[test]
    fn it_show_works_works() {
        let onig_string = OnigString::new(String::from("a💻bYX"));
        assert_eq!(8, onig_string.utf16length);
        assert_eq!(5, onig_string.utf8length);
    }
}
