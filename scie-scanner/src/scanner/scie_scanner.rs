use crate::scanner::onig_string::OnigString;
use crate::scanner::utf_string::UtfString;
use onigvs::{createOnigScanner, freeOnigScanner, findNextOnigScannerMatch, MAX_REGIONS, OnigScanner};
use std::os::raw::{c_int};

pub type Pointer = i32;

#[derive(Debug, Clone, Serialize)]
pub struct IOnigCaptureIndex {
    pub start: usize,
    pub end: usize,
    pub length: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct IOnigMatch {
    pub index: usize,
    pub capture_indices: Vec<IOnigCaptureIndex>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ScieScanner {
    #[serde(skip_serializing)]
    pub _ptr: *mut OnigScanner
}

pub type IntArray = Vec<i32>;

impl ScieScanner {
    pub fn new(patterns: Vec<String>) -> Self {
        let mut str_len_arr: Vec<c_int> = vec![0; patterns.len()];
        let mut str_ptrs_arr: Vec<*mut ::std::os::raw::c_uchar> = vec![];

        for i in 0..patterns.len() {
            let pattern = patterns[i].clone();
            let mut utf_string = UtfString::new(pattern);

            let mut _x = utf_string.createString();
            str_ptrs_arr.push(*&mut _x);
            str_len_arr[i] = utf_string.utf8length;
        }

        let onig_scanner;
        unsafe {
            let patterns_length_ptr = str_len_arr.as_mut_ptr();
            let patterns_ptr: *mut *mut ::std::os::raw::c_uchar = str_ptrs_arr.as_mut_ptr();
            onig_scanner = createOnigScanner(patterns_ptr, patterns_length_ptr, patterns.len() as i32);
        }

        ScieScanner { _ptr: onig_scanner as *mut OnigScanner }
    }

    pub fn dispose(&self) {
        unsafe {
            freeOnigScanner(self._ptr);
        }
    }

    pub fn find_next_match_sync(&mut self, string: String, start_position: i32) -> Option<IOnigMatch> {
        let mut onig_string = OnigString::new(string);
        let result = self._find_next_match_sync(&mut onig_string, start_position);
        onig_string.dispose();

        return result;
    }

    pub fn _find_next_match_sync(&self, string: &mut OnigString, start_position: i32) -> Option<IOnigMatch> {
        unsafe {
            let result = findNextOnigScannerMatch(
                self._ptr,
                string.id,
                string.ptr,
                string.utf8length,
                string.convertUtf16OffsetToUtf8(start_position),
            );

            if result == 0 {
                return None;
            }


            let index: usize;
            let mut capture_indices = vec![];

            let size = 2 * (1 + MAX_REGIONS);
            let result = std::slice::from_raw_parts(result as *const i32, size as usize);
            index = result[0] as usize;
            let count = result[1];
            let mut offset = 1;
            for _i in 0..count {
                offset = offset + 1;
                let start = string.convertUtf8OffsetToUtf16(result[offset]);
                offset = offset + 1;
                let end = string.convertUtf8OffsetToUtf16(result[offset]);
                let length = end - start;

                capture_indices.push(IOnigCaptureIndex {
                    start: start as usize,
                    end: end as usize,
                    length: length as usize,
                })
            }

            return Some(IOnigMatch {
                index,
                capture_indices,
            });
        }
    }
}

pub fn str_vec_to_string<I, T>(iter: I) -> Vec<String>
    where
        I: IntoIterator<Item=T>,
        T: Into<String>,
{
    iter.into_iter().map(Into::into).collect()
}

#[cfg(test)]
mod tests {
    use crate::scanner::scie_scanner::{ScieScanner, str_vec_to_string};

    #[test]
    fn should_init_onig_scanner() {
        let mut scanner = ScieScanner::new(vec![String::from("ell"), String::from("wo")]);
        let onig = scanner.clone().find_next_match_sync(String::from("z"), 1);
        assert!(onig.is_none());

        let onig2 = scanner.find_next_match_sync(String::from("Hello world!"), 0);
        assert_eq!(0, onig2.clone().unwrap().index);
        assert_eq!(1, onig2.clone().unwrap().capture_indices[0].start);
        assert_eq!(4, onig2.clone().unwrap().capture_indices[0].end);

        scanner.dispose();
    }

    #[test]
    fn should_handle_simple_regex() {
        let regex = vec![String::from("ell"), String::from("wo")];
        let mut scanner = ScieScanner::new(regex);
        let s = String::from("Hello world!");
        let result = scanner.find_next_match_sync(s.clone(), 0).unwrap();
        assert_eq!(result.index, 0);
        assert_eq!(result.capture_indices[0].start, 1);
        assert_eq!(result.capture_indices[0].end, 4);

        let second_result = scanner.find_next_match_sync(s, 2).unwrap();
        println!("second_result: {:?}", second_result);
        assert_eq!(second_result.index, 1);
        assert_eq!(second_result.capture_indices[0].start, 6);
        assert_eq!(second_result.capture_indices[0].end, 8);

        scanner.dispose();
    }

    #[test]
    fn should_handle_simple2() {
        let regex = vec![String::from("a"), String::from("b"), String::from("c")];
        let mut scanner = ScieScanner::new(regex);

        if let None = scanner.find_next_match_sync(String::from("x"), 0) {
            assert_eq!(true, true);
        } else {
            assert_eq!(true, false);
        }

        let result = scanner
            .find_next_match_sync(String::from("xxaxxbxxc"), 0)
            .unwrap();
        assert_eq!(
            serde_json::to_string(&result).unwrap(),
            String::from(
                "{\"index\":0,\"capture_indices\":[{\"start\":2,\"end\":3,\"length\":1}]}"
            )
        );

        let result2 = scanner
            .find_next_match_sync(String::from("xxaxxbxxc"), 4)
            .unwrap();
        assert_eq!(
            serde_json::to_string(&result2).unwrap(),
            String::from(
                "{\"index\":1,\"capture_indices\":[{\"start\":5,\"end\":6,\"length\":1}]}"
            )
        );

        let result3 = scanner
            .find_next_match_sync(String::from("xxaxxbxxc"), 7)
            .unwrap();
        assert_eq!(
            serde_json::to_string(&result3).unwrap(),
            String::from(
                "{\"index\":2,\"capture_indices\":[{\"start\":8,\"end\":9,\"length\":1}]}"
            )
        );

        if let None = scanner.find_next_match_sync(String::from("xxaxxbxxc"), 9) {
            assert_eq!(true, true);
        } else {
            assert_eq!(true, false);
        }

        scanner.dispose();
    }

    #[test]
    fn should_handle_unicode1() {
        let regex = vec![String::from("1"), String::from("2")];
        let mut scanner = ScieScanner::new(regex);

        let result = scanner
            .find_next_match_sync(String::from("ab…cde21"), 5)
            .unwrap();
        assert_eq!(
            serde_json::to_string(&result).unwrap(),
            String::from(
                "{\"index\":1,\"capture_indices\":[{\"start\":6,\"end\":7,\"length\":1}]}"
            )
        );

        scanner.dispose();
        let mut scanner2 = ScieScanner::new(vec![String::from("\"")]);
        let result2 = scanner2
            .find_next_match_sync(String::from("{\"…\": 1}"), 1)
            .unwrap();
        assert_eq!(
            serde_json::to_string(&result2).unwrap(),
            String::from(
                "{\"index\":0,\"capture_indices\":[{\"start\":1,\"end\":2,\"length\":1}]}"
            )
        );
        scanner2.dispose();
    }

    #[test]
    fn should_handle_unicode3() {
        let regex = vec![String::from("Y"), String::from("X")];
        let mut scanner = ScieScanner::new(regex);
        let result = scanner
            .find_next_match_sync(String::from("a💻bYX"), 0)
            .unwrap();
        assert_eq!(
            serde_json::to_string(&result).unwrap(),
            String::from(
                "{\"index\":0,\"capture_indices\":[{\"start\":4,\"end\":5,\"length\":1}]}"
            )
        );

        let result1 = scanner
            .find_next_match_sync(String::from("a💻bYX"), 1)
            .unwrap();
        assert_eq!(
            serde_json::to_string(&result1).unwrap(),
            String::from(
                "{\"index\":0,\"capture_indices\":[{\"start\":4,\"end\":5,\"length\":1}]}"
            )
        );

        let result2 = scanner
            .find_next_match_sync(String::from("a💻bYX"), 2)
            .unwrap();
        assert_eq!(
            serde_json::to_string(&result2).unwrap(),
            String::from(
                "{\"index\":0,\"capture_indices\":[{\"start\":4,\"end\":5,\"length\":1}]}"
            )
        );

        let result3 = scanner
            .find_next_match_sync(String::from("a💻bYX"), 3)
            .unwrap();
        assert_eq!(
            serde_json::to_string(&result3).unwrap(),
            String::from(
                "{\"index\":0,\"capture_indices\":[{\"start\":4,\"end\":5,\"length\":1}]}"
            )
        );

        let result4 = scanner
            .find_next_match_sync(String::from("a💻bYX"), 4)
            .unwrap();
        assert_eq!(
            serde_json::to_string(&result4).unwrap(),
            String::from(
                "{\"index\":0,\"capture_indices\":[{\"start\":4,\"end\":5,\"length\":1}]}"
            )
        );

        let result5 = scanner
            .find_next_match_sync(String::from("a💻bYX"), 5)
            .unwrap();
        assert_eq!(
            serde_json::to_string(&result5).unwrap(),
            String::from(
                "{\"index\":1,\"capture_indices\":[{\"start\":5,\"end\":6,\"length\":1}]}"
            )
        );

        scanner.dispose();
    }

    #[test]
    fn should_out_of_bounds() {
        let mut scanner = ScieScanner::new(vec![String::from("X")]);
        let result = scanner
            .find_next_match_sync(String::from("X💻X"), -10000)
            .unwrap();
        assert_eq!(
            serde_json::to_string(&result).unwrap(),
            String::from(
                "{\"index\":0,\"capture_indices\":[{\"start\":0,\"end\":1,\"length\":1}]}"
            )
        );

        let result2 = scanner.find_next_match_sync(String::from("X💻X"), 10000);
        assert!(result2.is_none());

        scanner.dispose();
    }

    #[test]
    fn should_handle_regex_g() {
        let mut scanner = ScieScanner::new(vec![String::from("\\G-and")]);
        let result = scanner.find_next_match_sync(String::from("first-and-second"), 0);
        assert_eq!(format!("{:?}", result), "None");

        let result2 = scanner
            .find_next_match_sync(String::from("first-and-second"), 5)
            .unwrap();
        assert_eq!(
            serde_json::to_string(&result2).unwrap(),
            String::from(
                "{\"index\":0,\"capture_indices\":[{\"start\":5,\"end\":9,\"length\":4}]}"
            )
        );

        scanner.dispose();
    }

    #[test]
    fn should_match_makefile_scan_regex() {
        let origin = vec![
            "(^[ \\t]+)?(?=#)",
            "(^[ ]*|\\G\\s*)([^\\s]+)\\s*(=|\\?=|:=|\\+=)",
            "^(?!\\t)([^:]*)(:)(?!\\=)",
            "^[ ]*([s\\-]?include)\\b",
            "^[ ]*(vpath)\\b",
            "^(?:(override)\\s*)?(define)\\s*([^\\s]+)\\s*(=|\\?=|:=|\\+=)?(?=\\s)",
            "^[ ]*(export)\\b",
            "^[ ]*(override|private)\\b",
            "^[ ]*(unexport|undefine)\\b",
            "^(ifdef|ifndef)\\s*([^\\s]+)(?=\\s)",
            "^(ifeq|ifneq)(?=\\s)]",
        ];
        let _rules = vec![2, 7, 28, 45, 48, 51, 61, 64, 66, 69, 77];
        let debug_regex = str_vec_to_string(origin);
        let mut scanner = ScieScanner::new(debug_regex);
        let result = scanner.find_next_match_sync(String::from("%.o: %.c $(DEPS)"), 0);
        assert_eq!(3, result.unwrap().capture_indices.len());

        scanner.dispose();
    }

    #[test]
    fn should_match_makefile_special_char() {
        let origin = vec!["(?=\\s|$)", "(\\$?\\$)[@%<?^+*]", "\\$?\\$\\(", "%"];
        let _rules = vec![-1, 12, 14, 33];
        let debug_regex = str_vec_to_string(origin);
        let mut scanner = ScieScanner::new(debug_regex);
        let result = scanner.find_next_match_sync(String::from("%.o"), 0);
        let onig_match = result.unwrap();
        assert_eq!(3, onig_match.index);
        assert_eq!(0, onig_match.clone().capture_indices[0].start);
        assert_eq!(1, onig_match.clone().capture_indices[0].end);

        scanner.dispose();
    }

    #[test]
    fn should_match_for_scope_target() {
        let origin = vec!["^(?!\\t)", "\\G", "^\\t"];
        let _rules = vec![-1, 36, 39];
        let debug_regex = str_vec_to_string(origin);
        let mut scanner = ScieScanner::new(debug_regex);
        let result = scanner.find_next_match_sync(
            String::from(
                "%.o: %.c $(DEPS)
    ",
            ),
            4,
        );
        let onig_match = result.unwrap();
        assert_eq!(1, onig_match.index);
        assert_eq!(4, onig_match.capture_indices[0].start);
        assert_eq!(4, onig_match.capture_indices[0].end);

        scanner.dispose();
    }

    #[test]
    fn should_return_correct_index_when_for_markdown() {
        let origin = vec![
            "^",
            "\\\n",
            "%|\\*",
            "(^[ \t]+)?(?=#)",
            "(\\$?\\$)[@%<?^+*]",
            "\\$?\\$\\(",
        ];
        let _rules = vec![-1, 37, 38, 2, 12, 14];
        let debug_regex = str_vec_to_string(origin);
        let mut scanner = ScieScanner::new(debug_regex);
        let result = scanner.find_next_match_sync(
            String::from(
                "%.o: %.c $(DEPS)
    ",
            ),
            4,
        );
        let onig_match = result.unwrap();
        assert_eq!(2, onig_match.index);
        assert_eq!(5, onig_match.capture_indices[0].start);
        assert_eq!(6, onig_match.capture_indices[0].end);

        scanner.dispose();
    }

    #[test]
    fn should_return_null_when_out_size() {
        let origin = vec![
            "^",
            "\\\n",
            "%|\\*",
            "(^[ \t]+)?(?=#)",
            "(\\$?\\$)[@%<?^+*]",
            "\\$?\\$\\(",
        ];
        let _rules = vec![-1, 37, 38, 2, 12, 14];
        let debug_regex = str_vec_to_string(origin);
        let mut scanner = ScieScanner::new(debug_regex);
        let result = scanner.find_next_match_sync(String::from("%.o: %.c $(DEPS)"), 16);
        assert!(result.is_none());

        scanner.dispose();
    }
}
