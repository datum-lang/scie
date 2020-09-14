use onigvs::malloc;

#[derive(Clone, Debug)]
pub struct UtfString {
    pub utf16length: i32,
    pub utf8length: i32,
    pub utf16value: String,
    pub utf8value: Vec<u8>,
    pub utf16offset_to_utf8: Vec<u32>,
    pub utf8offset_to_utf16: Vec<u32>,
}

impl UtfString {
    pub fn new(str: String) -> Self {
        let utf16_vec: Vec<u16> = str.clone().encode_utf16().collect();
        let utf16length = utf16_vec.len();
        let utf8length = str.len();
        let mut utf8value = str.clone().into_bytes();

        let compute_indices_mapping = utf8length != utf16length;

        let mut utf16offset_to_utf8: Vec<u32> = vec![];
        let mut utf8offset_to_utf16: Vec<u32> = vec![];

        if compute_indices_mapping {
            utf16offset_to_utf8 = vec![0; utf16length.clone() + 1];
            utf16offset_to_utf8[utf16length] = utf8length.clone() as u32;

            utf8offset_to_utf16 = vec![0; utf8length.clone() + 1];
            utf8offset_to_utf16[utf8length] = utf16length.clone() as u32;
        }

        let mut i8: usize = 0;
        let mut i16 = 0;
        while i16 < utf16_vec.len() {
            let char_code = utf16_vec[i16].clone();
            let mut code_point = char_code.clone() as usize;
            let mut was_surrogate_pair = false;
            if char_code >= 0xd800 && char_code <= 0xdbff {
                // todo: update logic
                // was_surrogate_pair = true;
                if i16 + 1 <= utf16length {
                    let next_char_code = utf16_vec[i16 + 1].clone();
                    if next_char_code >= 0xdc00 && next_char_code <= 0xdfff {
                        let temp = ((char_code - 0xd800) << 10) as usize + 0x10000;
                        code_point = (temp as usize) | (next_char_code as usize - 0xdc00);
                        was_surrogate_pair = true;
                    }
                }
            }

            if compute_indices_mapping {
                utf16offset_to_utf8[i16] = i8 as u32;

                if was_surrogate_pair {
                    utf16offset_to_utf8[i16 + 1] = i8 as u32;
                }

                if code_point <= 0x7f {
                    utf8offset_to_utf16[i8 + 0] = i16 as u32;
                } else if code_point <= 0x7ff {
                    utf8offset_to_utf16[i8 + 0] = i16 as u32;
                    utf8offset_to_utf16[i8 + 1] = i16 as u32;
                } else if code_point <= 0xffff {
                    utf8offset_to_utf16[i8 + 0] = i16 as u32;
                    utf8offset_to_utf16[i8 + 1] = i16 as u32;
                    utf8offset_to_utf16[i8 + 2] = i16 as u32;
                } else {
                    utf8offset_to_utf16[i8 + 0] = i16 as u32;
                    utf8offset_to_utf16[i8 + 1] = i16 as u32;
                    utf8offset_to_utf16[i8 + 2] = i16 as u32;
                    utf8offset_to_utf16[i8 + 3] = i16 as u32;
                }
            }

            if code_point <= 0x7f {
                utf8value[i8] = code_point as u8;
                i8 = i8 + 1;
            } else if code_point <= 0x7ff {
                utf8value[i8] =
                    (0b11000000 | ((code_point & 0b00000000000000000000011111000000) >> 6)) as u8;
                i8 = i8 + 1;
                utf8value[i8] =
                    (0b10000000 | ((code_point & 0b00000000000000000000000000111111) >> 0)) as u8;
                i8 = i8 + 1;
            } else if code_point <= 0xffff {
                utf8value[i8] =
                    (0b11100000 | ((code_point & 0b00000000000000001111000000000000) >> 12)) as u8;
                i8 = i8 + 1;
                utf8value[i8] =
                    (0b10000000 | ((code_point & 0b00000000000000000000111111000000) >> 6)) as u8;
                i8 = i8 + 1;
                utf8value[i8] =
                    (0b10000000 | ((code_point & 0b00000000000000000000000000111111) >> 0)) as u8;
                i8 = i8 + 1;
            } else {
                utf8value[i8] =
                    (0b11110000 | ((code_point & 0b00000000000111000000000000000000) >> 18)) as u8;
                i8 = i8 + 1;
                utf8value[i8] =
                    (0b10000000 | ((code_point & 0b00000000000000111111000000000000) >> 12)) as u8;
                i8 = i8 + 1;
                utf8value[i8] =
                    (0b10000000 | ((code_point & 0b00000000000000000000111111000000) >> 6)) as u8;
                i8 = i8 + 1;
                utf8value[i8] =
                    (0b10000000 | ((code_point & 0b00000000000000000000000000111111) >> 0)) as u8;
                i8 = i8 + 1;
            }

            if was_surrogate_pair {
                i16 = i16 + 1;
            }

            i16 = i16 + 1;
        }

        UtfString {
            utf16length: utf16length as i32,
            utf8length: utf8length as i32,
            utf16value: str.clone(),
            utf8value,
            utf16offset_to_utf8,
            utf8offset_to_utf16,
        }
    }

    pub fn createString(&mut self) -> *mut u8 {
        let result: *mut u8;
        unsafe {
            result = malloc(self.utf8length as u64) as *mut u8;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::scanner::utf_string::UtfString;
    use crate::scanner::scie_scanner::{ScieScanner, str_vec_to_string};
    use onigvs::malloc;

    #[test]
    fn should_convert_utf_string_success() {
        let onig_string = UtfString::new(String::from("a💻bYX"));

        assert_eq!(6, onig_string.utf16length);
        assert_eq!(8, onig_string.utf8length);
        assert_eq!(
            vec![97, 240, 159, 146, 187, 98, 89, 88],
            onig_string.utf8value
        );
        assert_eq!(vec![0, 1, 1, 5, 6, 7, 8], onig_string.utf16offset_to_utf8);
        assert_eq!(
            vec![0, 1, 1, 1, 1, 3, 4, 5, 6],
            onig_string.utf8offset_to_utf16
        );
    }

    #[test]
    fn should_handle_normal_String() {
        let onig_string = UtfString::new(String::from("12"));
        assert_eq!(2, onig_string.utf16length.clone());
        assert_eq!(2, onig_string.utf8length.clone());
    }

    #[test]
    fn should_create_string_success() {
        let long_str = "\\b(AbsoluteTime|Boolean|Byte|ByteCount|ByteOffset|BytePtr|CompTimeValue|ConstLogicalAddress|ConstStrFileNameParam|ConstStringPtr|Duration|Fixed|FixedPtr|Float32|Float32Point|Float64|Float80|Float96|FourCharCode|Fract|FractPtr|Handle|ItemCount|LogicalAddress|OptionBits|OSErr|OSStatus|OSType|OSTypePtr|PhysicalAddress|ProcessSerialNumber|ProcessSerialNumberPtr|ProcHandle|Ptr|ResType|ResTypePtr|ShortFixed|ShortFixedPtr|SignedByte|SInt16|SInt32|SInt64|SInt8|Size|StrFileName|StringHandle|StringPtr|TimeBase|TimeRecord|TimeScale|TimeValue|TimeValue64|UInt16|UInt32|UInt64|UInt8|UniChar|UniCharCount|UniCharCountPtr|UniCharPtr|UnicodeScalarValue|UniversalProcHandle|UniversalProcPtr|UnsignedFixed|UnsignedFixedPtr|UnsignedWide|UTF16Char|UTF32Char|UTF8Char)\\b";
        let mut utf_string = UtfString::new(String::from(long_str));
        unsafe {
            let x = malloc(utf_string.utf8length as u64) as *mut u32;
            assert_ne!(*x, 0);
        }
    }

    struct NormalString {
        pub length: i32,
        pub str: String
    }

    impl NormalString {
        pub fn new(str: String) -> Self {
            NormalString { length: str.clone().len() as i32, str }
        }
    }

    #[test]
    fn should_create_string_with_struct_success() {
        let long_str = "\\b(AbsoluteTime|Boolean|Byte|ByteCount|ByteOffset|BytePtr|CompTimeValue|ConstLogicalAddress|ConstStrFileNameParam|ConstStringPtr|Duration|Fixed|FixedPtr|Float32|Float32Point|Float64|Float80|Float96|FourCharCode|Fract|FractPtr|Handle|ItemCount|LogicalAddress|OptionBits|OSErr|OSStatus|OSType|OSTypePtr|PhysicalAddress|ProcessSerialNumber|ProcessSerialNumberPtr|ProcHandle|Ptr|ResType|ResTypePtr|ShortFixed|ShortFixedPtr|SignedByte|SInt16|SInt32|SInt64|SInt8|Size|StrFileName|StringHandle|StringPtr|TimeBase|TimeRecord|TimeScale|TimeValue|TimeValue64|UInt16|UInt32|UInt64|UInt8|UniChar|UniCharCount|UniCharCountPtr|UniCharPtr|UnicodeScalarValue|UniversalProcHandle|UniversalProcPtr|UnsignedFixed|UnsignedFixedPtr|UnsignedWide|UTF16Char|UTF32Char|UTF8Char)\\b";
        let mut utf_string = NormalString::new(String::from(long_str));
        unsafe {
            let x = malloc(utf_string.length as u64) as *mut u32;
            assert_ne!(*x, 0);
        }
    }

    #[test]
    fn should_convert_string_list() {
        let lists = vec![
            "^\\s*(#(if)\\s+(0*1)\\b)",
            "^\\s*(#(if)\\s+(0)\\b).*",
            "^\\s*(#\\s*(if(n?def)?)\\b.*?(?:(?=(?://|/\\*))|$))",
            "^/\\* =(\\s*.*?)\\s*= \\*/$\\n?",
            "/\\*",
            "\\*/.*\\n",
            "^// =(\\s*.*?)\\s*=\\s*$\\n?",
            "(^[ \\t]+)?(?=//)",
            "\\b(break|case|continue|default|do|else|for|goto|if|_Pragma|return|switch|while)\\b",
            "\\b(asm|__asm__|auto|bool|_Bool|char|_Complex|double|enum|float|_Imaginary|int|long|short|signed|struct|typedef|union|unsigned|void)\\b",
            "\\b(const|extern|register|restrict|static|volatile|inline)\\b",
            "\\bk[A-Z]\\w*\\b",
            "\\bg[A-Z]\\w*\\b",
            "\\bs[A-Z]\\w*\\b",
            "\\b(NULL|true|false|TRUE|FALSE)\\b",
            "\\b(sizeof)\\b",
            "\\b((0(x|X)[0-9a-fA-F]*)|(([0-9]+\\.?[0-9]*)|(\\.[0-9]+))((e|E)(\\+|-)?[0-9]+)?)(L|l|UL|ul|u|U|F|f|ll|LL|ull|ULL)?\\b",
            "\"",
            "'",
            "(?x)\\n        \\t\\t^\\s*\\#\\s*(define)\\s+             # define\\n        \\t\\t((?<id>[a-zA-Z_][a-zA-Z0-9_]*))  # macro name\\n        \\t\\t(?:                              # and optionally:\\n        \\t\\t    (\\()                         # an open parenthesis\\n        \\t\\t        (\\n        \\t\\t            \\s* \\g<id> \\s*       # first argument\\n        \\t\\t            ((,) \\s* \\g<id> \\s*)*  # additional arguments\\n        \\t\\t            (?:\\.\\.\\.)?          # varargs ellipsis?\\n        \\t\\t        )\\n        \\t\\t    (\\))                         # a close parenthesis\\n        \\t\\t)?\\n        \\t",
            "^\\s*#\\s*(error|warning)\\b",
            "^\\s*#\\s*(include|import)\\b\\s+",
            "^\\s*(#\\s*(pragma\\s+mark)\\s+(.*))",
            "^\\s*#\\s*(define|defined|elif|else|if|ifdef|ifndef|line|pragma|undef)\\b",
            "\\b(u_char|u_short|u_int|u_long|ushort|uint|u_quad_t|quad_t|qaddr_t|caddr_t|daddr_t|dev_t|fixpt_t|blkcnt_t|blksize_t|gid_t|in_addr_t|in_port_t|ino_t|key_t|mode_t|nlink_t|id_t|pid_t|off_t|segsz_t|swblk_t|uid_t|id_t|clock_t|size_t|ssize_t|time_t|useconds_t|suseconds_t)\\b",
            "\\b(pthread_attr_t|pthread_cond_t|pthread_condattr_t|pthread_mutex_t|pthread_mutexattr_t|pthread_once_t|pthread_rwlock_t|pthread_rwlockattr_t|pthread_t|pthread_key_t)\\b",
            "\\b(int8_t|int16_t|int32_t|int64_t|uint8_t|uint16_t|uint32_t|uint64_t|int_least8_t|int_least16_t|int_least32_t|int_least64_t|uint_least8_t|uint_least16_t|uint_least32_t|uint_least64_t|int_fast8_t|int_fast16_t|int_fast32_t|int_fast64_t|uint_fast8_t|uint_fast16_t|uint_fast32_t|uint_fast64_t|intptr_t|uintptr_t|intmax_t|intmax_t|uintmax_t|uintmax_t)\\b",
            "\\b(noErr|kNilOptions|kInvalidID|kVariableLengthArray)\\b",
            "\\b(AbsoluteTime|Boolean|Byte|ByteCount|ByteOffset|BytePtr|CompTimeValue|ConstLogicalAddress|ConstStrFileNameParam|ConstStringPtr|Duration|Fixed|FixedPtr|Float32|Float32Point|Float64|Float80|Float96|FourCharCode|Fract|FractPtr|Handle|ItemCount|LogicalAddress|OptionBits|OSErr|OSStatus|OSType|OSTypePtr|PhysicalAddress|ProcessSerialNumber|ProcessSerialNumberPtr|ProcHandle|Ptr|ResType|ResTypePtr|ShortFixed|ShortFixedPtr|SignedByte|SInt16|SInt32|SInt64|SInt8|Size|StrFileName|StringHandle|StringPtr|TimeBase|TimeRecord|TimeScale|TimeValue|TimeValue64|UInt16|UInt32|UInt64|UInt8|UniChar|UniCharCount|UniCharCountPtr|UniCharPtr|UnicodeScalarValue|UniversalProcHandle|UniversalProcPtr|UnsignedFixed|UnsignedFixedPtr|UnsignedWide|UTF16Char|UTF32Char|UTF8Char)\\b",
            "\\b([a-z0-9_]+_t)\\b",
            "\\{",
            "(?x)\\n    \\t\\t(?:  ^                                 # begin-of-line\\n    \\t\\t  |  \\n    \\t\\t     (?: (?= \\s )           (?<!else|new|return) (?<=\\w)      #  or word + space before name\\n    \\t\\t       | (?= \\s*[A-Za-z_] ) (?<!&&)       (?<=[*&>])   #  or type modifier before name\\n    \\t\\t     )\\n    \\t\\t)\\n    \\t\\t(\\s*) (?!(while|for|do|if|else|switch|catch|enumerate|return|sizeof|[cr]?iterate)\\s*\\()\\n    \\t\\t(\\n    \\t\\t\\t(?: [A-Za-z_][A-Za-z0-9_]*+ | :: )++ |                  # actual name\\n    \\t\\t\\t(?: (?<=operator) (?: [-*&<>=+!]+ | \\(\\) | \\[\\] ) )  # if it is a C++ operator\\n    \\t\\t)\\n    \\t\\t \\s*(?=\\()",
        ];
        let debug_regex = str_vec_to_string(lists);
        for x in debug_regex {
            let utf_string = UtfString::new(x.clone());
            assert_eq!(x, String::from_utf8_lossy(&*utf_string.utf8value));
        };
    }
}
