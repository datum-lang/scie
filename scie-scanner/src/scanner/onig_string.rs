use crate::scanner::utf_string::UtfString;
use onigvs::free;
use std::ffi::c_void;

#[derive(Debug, Clone)]
pub struct OnigString {
    pub id: i32,
    pub content: String,
    pub utf16length: i32,
    pub utf8length: i32,
    pub ptr: *mut i32,
    pub utf16offset_to_utf8: Vec<u32>,
    pub utf8offset_to_utf16: Vec<u32>,
}

impl OnigString {
    pub fn new(str: String) -> Self {
        let mut utf_string = UtfString::new(str.clone());
        let ptr = utf_string.createString();
        let onig_string = OnigString {
            id: 0,
            content: str.to_string(),
            utf16length: utf_string.utf16length,
            utf8length: utf_string.utf8length,
            ptr: ptr,
            utf16offset_to_utf8: utf_string.utf16offset_to_utf8,
            utf8offset_to_utf16: utf_string.utf8offset_to_utf16,
        };

        onig_string
    }

    pub fn dispose(&self) {
        unsafe {
            free(self.ptr as *mut c_void);
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
                return 0
            }
            if utf16Offset > self.utf16length {
                return self.utf8length
            }

            return self.utf16offset_to_utf8[utf16Offset as usize] as i32;
        }

        return utf16Offset
    }
}
