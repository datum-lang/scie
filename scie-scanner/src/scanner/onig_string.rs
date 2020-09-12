use crate::scanner::utf_string::UtfString;

pub struct OnigString {
    pub id: i32,
    pub content: String,
    pub utf16length: i32,
    pub utf8length: i32,
    pub ptr: i32,
    pub utf16offset_to_utf8: Vec<u32>,
    pub utf8offset_to_utf16: Vec<u32>,
}

impl OnigString {
    pub fn new(str: String) -> Self {
        // let utf_string = UtfString::createString();
        OnigString {
            id: 0,
            content: str.to_string(),
            utf16length: 0,
            utf8length: 0,
            ptr: 0,
            utf16offset_to_utf8: vec![],
            utf8offset_to_utf16: vec![],
        }
    }

    pub fn convertUtf8OffsetToUtf16(&self, utf8Offset: i32) -> i32 {
        0
    }
}
