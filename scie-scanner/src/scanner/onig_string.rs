use crate::scanner::utf_string::UtfString;

#[derive(Debug, Clone)]
pub struct OnigString<'a> {
    pub id: i32,
    pub content: &'a str,
    pub utf16length: i32,
    pub utf8length: i32,
    pub utf16offset_to_utf8: Vec<u32>,
    pub utf8offset_to_utf16: Vec<u32>,
}

impl<'a> OnigString<'a> {
    pub fn new(str: &str, id: i32) -> OnigString {
        let utf_string = UtfString::new(str);
        OnigString {
            id,
            content: str,
            utf16length: utf_string.utf16length,
            utf8length: utf_string.utf8length,
            utf16offset_to_utf8: utf_string.utf16offset_to_utf8,
            utf8offset_to_utf16: utf_string.utf8offset_to_utf16,
        }
    }

    pub fn convertUtf8OffsetToUtf16(&self, utf8Offset: i32) -> i32 {
        if self.utf8offset_to_utf16.len() > 0 {
            if utf8Offset < 0 {
                return 0;
            }
            if utf8Offset > self.utf8length {
                return self.utf16length;
            }
            return self.utf8offset_to_utf16[utf8Offset as usize] as i32;
        }
        return utf8Offset;
    }

    pub fn convertUtf16OffsetToUtf8(&self, utf16Offset: i32) -> i32 {
        if self.utf16offset_to_utf8.len() > 0 {
            if utf16Offset < 0 {
                return 0;
            }
            if utf16Offset > self.utf16length {
                return self.utf8length;
            }

            return self.utf16offset_to_utf8[utf16Offset as usize] as i32;
        }

        return utf16Offset;
    }
}

#[cfg(test)]
mod tests {
    use crate::scanner::onig_string::OnigString;

    #[test]
    fn should_handle_offset() {
        let onig_string = OnigString::new("a💻bYX", 1);
        let x = onig_string.convertUtf8OffsetToUtf16(2);
        assert_eq!(1, x);
    }
}
