use onig::Regex;
use unicode_segmentation::UnicodeSegmentation;

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

#[derive(Debug, Clone, Serialize)]
pub struct Scanner {
    pub index: usize,
    pub patterns: Vec<String>,
}

impl Scanner {
    pub fn new(patterns: Vec<String>) -> Self {
        Scanner { index: 0, patterns }
    }

    pub fn dispose(&mut self) {
        self.index = 0
    }

    pub fn find_next_match_sync(
        &mut self,
        origin_str: String,
        start_position: i32,
    ) -> Option<IOnigMatch> {
        if self.index >= self.patterns.clone().len() {
            self.index = 0;
            return None;
        }

        let mut all_results: Vec<IOnigMatch> = vec![];
        for (index, pattern) in self.patterns.iter().enumerate() {
            let mut after_pos_str = String::from("");
            let mut start_pos = start_position;
            let string_vec = origin_str.graphemes(true).collect::<Vec<&str>>();

            if start_pos >= string_vec.len() as i32 {
                return None;
            }

            if start_pos < 0 {
                start_pos = 0
            }

            let before_vec = string_vec[..start_pos as usize].to_owned();
            let after_vec = string_vec[start_pos as usize..].to_owned();

            for x in after_vec {
                after_pos_str = after_pos_str + x
            }

            let _regex = Regex::new(pattern.as_str());
            if let Err(_err) = _regex {
                return None;
            }

            let regex = _regex.unwrap();
            let mut capture_indices = vec![];
            let _captures = regex.captures(after_pos_str.as_str());

            if let Some(captures) = _captures {
                for (_, pos) in captures.iter_pos().enumerate() {
                    if let Some((start, end)) = pos {
                        let length = end - start;
                        let x1 = after_pos_str.split_at(end).0;
                        let utf8_end =
                            before_vec.len() + x1.graphemes(true).collect::<Vec<&str>>().len();
                        let utf8_start = utf8_end - length;

                        let capture = IOnigCaptureIndex {
                            start: utf8_start,
                            end: utf8_end,
                            length,
                        };

                        capture_indices.push(capture);
                    }
                }

                all_results.push(IOnigMatch {
                    index,
                    capture_indices,
                })
            }
        }

        if all_results.len() > 0 {
            let mut best_match = all_results[0].clone();
            for x in all_results {
                // todo: maybe have multiple captures
                if x.capture_indices[0].start <= best_match.capture_indices[0].start {
                    best_match = x;
                }
            }
            Some(best_match.clone())
        } else {
            None
        }
    }
}

pub fn str_vec_to_string<I, T>(iter: I) -> Vec<String>
where
    I: IntoIterator<Item = T>,
    T: Into<String>,
{
    iter.into_iter().map(Into::into).collect()
}

#[cfg(test)]
mod tests {
    use crate::scanner::scanner::{str_vec_to_string, IOnigMatch, Scanner};

    #[test]
    fn should_handle_simple_regex() {
        let regex = vec![String::from("ell"), String::from("wo")];
        let mut scanner = Scanner::new(regex);
        let s = String::from("Hello world!");
        let result = scanner.find_next_match_sync(s.clone(), 0).unwrap();
        assert_eq!(result.index, 0);
        assert_eq!(result.capture_indices[0].start, 1);
        assert_eq!(result.capture_indices[0].end, 4);

        let second_result = scanner.find_next_match_sync(s, 2).unwrap();
        assert_eq!(second_result.index, 1);
        assert_eq!(second_result.capture_indices[0].start, 6);
        assert_eq!(second_result.capture_indices[0].end, 8);
    }

    #[test]
    fn should_handle_simple2() {
        let regex = vec![String::from("a"), String::from("b"), String::from("c")];
        let mut scanner = Scanner::new(regex);

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
    }

    #[test]
    fn should_handle_unicode1() {
        let regex = vec![String::from("1"), String::from("2")];
        let mut scanner = Scanner::new(regex);

        let result = scanner
            .find_next_match_sync(String::from("ab…cde21"), 5)
            .unwrap();
        assert_eq!(
            serde_json::to_string(&result).unwrap(),
            String::from(
                "{\"index\":1,\"capture_indices\":[{\"start\":6,\"end\":7,\"length\":1}]}"
            )
        );
    }

    #[test]
    fn should_handle_unicode2() {
        let mut scanner2 = Scanner::new(vec![String::from("\"")]);
        let result2 = scanner2
            .find_next_match_sync(String::from("{\"…\": 1}"), 1)
            .unwrap();
        assert_eq!(
            serde_json::to_string(&result2).unwrap(),
            String::from(
                "{\"index\":0,\"capture_indices\":[{\"start\":1,\"end\":2,\"length\":1}]}"
            )
        );
    }

    #[test]
    fn should_handle_unicode3() {
        let regex = vec![String::from("Y"), String::from("X")];
        let mut scanner = Scanner::new(regex);
        let result = scanner
            .find_next_match_sync(String::from("a💻bYX"), 0)
            .unwrap();
        assert_eq!(
            serde_json::to_string(&result).unwrap(),
            String::from(
                "{\"index\":0,\"capture_indices\":[{\"start\":3,\"end\":4,\"length\":1}]}"
            )
        );

        let result1 = scanner
            .find_next_match_sync(String::from("a💻bYX"), 1)
            .unwrap();
        assert_eq!(
            serde_json::to_string(&result1).unwrap(),
            String::from(
                "{\"index\":0,\"capture_indices\":[{\"start\":3,\"end\":4,\"length\":1}]}"
            )
        );

        let result2 = scanner
            .find_next_match_sync(String::from("a💻bYX"), 2)
            .unwrap();
        assert_eq!(
            serde_json::to_string(&result2).unwrap(),
            String::from(
                "{\"index\":0,\"capture_indices\":[{\"start\":3,\"end\":4,\"length\":1}]}"
            )
        );

        let result3 = scanner
            .find_next_match_sync(String::from("a💻bYX"), 3)
            .unwrap();
        assert_eq!(
            serde_json::to_string(&result3).unwrap(),
            String::from(
                "{\"index\":0,\"capture_indices\":[{\"start\":3,\"end\":4,\"length\":1}]}"
            )
        );

        let result4 = scanner
            .find_next_match_sync(String::from("a💻bYX"), 4)
            .unwrap();
        assert_eq!(
            serde_json::to_string(&result4).unwrap(),
            String::from(
                "{\"index\":1,\"capture_indices\":[{\"start\":4,\"end\":5,\"length\":1}]}"
            )
        );
    }

    #[test]
    fn should_out_of_bounds() {
        let mut scanner = Scanner::new(vec![String::from("X")]);
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
        assert_eq!(format!("{:?}", result2), "None");
    }

    #[test]
    fn should_handle_regex_g() {
        let mut scanner = Scanner::new(vec![String::from("\\G-and")]);
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
        let rules = vec![2, 7, 28, 45, 48, 51, 61, 64, 66, 69, 77];
        let debug_regex = str_vec_to_string(origin);
        let mut scanner = Scanner::new(debug_regex);
        let result = scanner.find_next_match_sync(String::from("%.o: %.c $(DEPS)"), 0);
        assert_eq!(3, result.unwrap().capture_indices.len());
    }

    #[test]
    fn should_match_makefile_special_char() {
        let origin = vec!["(?=\\s|$)", "(\\$?\\$)[@%<?^+*]", "\\$?\\$\\(", "%"];
        let rules = vec![-1, 12, 14, 33];
        let debug_regex = str_vec_to_string(origin);
        let mut scanner = Scanner::new(debug_regex);
        let result = scanner.find_next_match_sync(String::from("%.o"), 0);
        let onig_match = result.unwrap();
        assert_eq!(3, onig_match.index);
        assert_eq!(0, onig_match.clone().capture_indices[0].start);
        assert_eq!(1, onig_match.clone().capture_indices[0].end);
    }

    #[test]
    fn should_match_for_scope_target() {
        let origin = vec!["^(?!\\t)", "\\G", "^\\t"];
        let rules = vec![-1, 36, 39];
        let debug_regex = str_vec_to_string(origin);
        let mut scanner = Scanner::new(debug_regex);
        let result = scanner.find_next_match_sync(String::from("%.o: %.c $(DEPS)
"), 4);
        let onig_match = result.unwrap();
        assert_eq!(1, onig_match.index);
        assert_eq!(4, onig_match.capture_indices[0].start);
        assert_eq!(4, onig_match.capture_indices[0].end);
    }
}
