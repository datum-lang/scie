use onig::{Regex};
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

#[derive(Debug, Clone)]
pub struct Scanner {
    pub index: usize,
    pub patterns: Vec<String>,
}

impl Scanner {
    pub fn new(patterns: Vec<String>) -> Self {
        Scanner {
            index: 0,
            patterns,
        }
    }

    pub fn dispose(&mut self) {
        self.index = 0
    }

    pub fn find_next_match_sync(&mut self, origin_str: String, start_position: i32) -> Option<IOnigMatch> {
        if self.index >= self.patterns.clone().len() {
            self.index = 0;
            return None;
        }

        let mut match_str = String::from("");
        let mut start_pos = start_position;
        let mut match_vec = origin_str.graphemes(true).collect::<Vec<&str>>();

        if start_pos > match_vec.len() as i32 {
            return None;
        }

        if start_pos >= 0 {
            match_vec = match_vec[start_pos as usize..].to_owned();
            for x in match_vec {
                match_str = match_str + x
            }
        } else {
            match_str = origin_str.clone();
            start_pos = 0
        }

        let pattern = self.patterns[self.index].clone();

        let regex = Regex::new(pattern.as_str()).unwrap();
        let mut capture_indices = vec![];
        let _captures = regex.captures(match_str.as_str());

        if let Some(captures) = _captures {
            for (_, pos) in captures.iter_pos().enumerate() {
                if let Some((start, end)) = pos {
                    let capture = IOnigCaptureIndex {
                        start: start_pos as usize + start,
                        end: start_pos as usize + end,
                        length: end - start,
                    };
                    capture_indices.push(capture)
                }
            }
        }

        if capture_indices.len() <= 0 {
            self.index = self.index + 1;
            self.find_next_match_sync(origin_str.clone(), start_pos)
        } else {
            let index = self.index.clone();
            self.index = 0;
            Some(IOnigMatch {
                index,
                capture_indices,
            })
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::scanner::scanner::{Scanner, IOnigMatch};

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

        let result = scanner.find_next_match_sync(String::from("xxaxxbxxc"), 0).unwrap();
        assert_eq!(serde_json::to_string(&result).unwrap(), String::from("{\"index\":0,\"capture_indices\":[{\"start\":2,\"end\":3,\"length\":1}]}"));

        let result2 = scanner.find_next_match_sync(String::from("xxaxxbxxc"), 4).unwrap();
        assert_eq!(serde_json::to_string(&result2).unwrap(), String::from("{\"index\":1,\"capture_indices\":[{\"start\":5,\"end\":6,\"length\":1}]}"));

        let result3 = scanner.find_next_match_sync(String::from("xxaxxbxxc"), 7).unwrap();
        assert_eq!(serde_json::to_string(&result3).unwrap(), String::from("{\"index\":2,\"capture_indices\":[{\"start\":8,\"end\":9,\"length\":1}]}"));

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

        let result = scanner.find_next_match_sync(String::from("ab…cde21"), 5).unwrap();
        assert_eq!(serde_json::to_string(&result).unwrap(), String::from("{\"index\":0,\"capture_indices\":[{\"start\":7,\"end\":8,\"length\":1}]}"));
    }

    #[test]
    fn should_handle_unicode2() {
        let mut scanner2 = Scanner::new(vec![String::from("\"")]);
        let result2 = scanner2.find_next_match_sync(String::from("{\"…\": 1}"), 1).unwrap();
        assert_eq!(serde_json::to_string(&result2).unwrap(), String::from("{\"index\":0,\"capture_indices\":[{\"start\":1,\"end\":2,\"length\":1}]}"));
    }

    #[test]
    fn should_handle_unicode3() {
        let regex = vec![String::from("Y"), String::from("X")];
        let mut scanner = Scanner::new(regex);
        let result = scanner.find_next_match_sync(String::from("a💻bYX"), 0).unwrap();
        assert_eq!(serde_json::to_string(&result).unwrap(), String::from("{\"index\":0,\"capture_indices\":[{\"start\":6,\"end\":7,\"length\":1}]}"));
        //
        // let result1 = scanner.find_next_match_sync(String::from("a💻bYX"), 1).unwrap();
        // assert_eq!(serde_json::to_string(&result1).unwrap(), String::from("{\"index\":0,\"capture_indices\":[{\"start\":3,\"end\":4,\"length\":1}]}"));
        //
        // let result2 = scanner.find_next_match_sync(String::from("a💻bYX"), 2).unwrap();
        // assert_eq!(serde_json::to_string(&result2).unwrap(), String::from("{\"index\":0,\"capture_indices\":[{\"start\":3,\"end\":4,\"length\":1}]}"));
        //
        // let result3 = scanner.find_next_match_sync(String::from("a💻bYX"), 3).unwrap();
        // assert_eq!(serde_json::to_string(&result3).unwrap(), String::from("{\"index\":0,\"capture_indices\":[{\"start\":3,\"end\":4,\"length\":1}]}"));

        // let result4 = scanner.find_next_match_sync(String::from("a💻bYX"), 4).unwrap();
        // assert_eq!(serde_json::to_string(&result4).unwrap(), String::from("{\"index\":0,\"capture_indices\":[{\"start\":6,\"end\":7,\"length\":1}]}"));
        //
        // let result5 = scanner.find_next_match_sync(String::from("a💻bYX"), 5).unwrap();
        // assert_eq!(serde_json::to_string(&result5).unwrap(), String::from("{\"index\":0,\"capture_indices\":[{\"start\":6,\"end\":7,\"length\":1}]}"));
        //
        // let result6 = scanner.find_next_match_sync(String::from("a💻bYX"), 6).unwrap();
        // assert_eq!(serde_json::to_string(&result6).unwrap(), String::from("{\"index\":0,\"capture_indices\":[{\"start\":6,\"end\":7,\"length\":1}]}"));
        //
        // let result7 = scanner.find_next_match_sync(String::from("a💻bYX"), 7).unwrap();
        // assert_eq!(serde_json::to_string(&result7).unwrap(), String::from("{\"index\":1,\"capture_indices\":[{\"start\":7,\"end\":8,\"length\":1}]}"));
    }

    #[test]
    fn should_out_of_bounds() {
        let mut scanner = Scanner::new(vec![String::from("X")]);
        let result = scanner.find_next_match_sync(String::from("X💻X"), -10000).unwrap();
        assert_eq!(serde_json::to_string(&result).unwrap(), String::from("{\"index\":0,\"capture_indices\":[{\"start\":0,\"end\":1,\"length\":1}]}"));

        let result2 = scanner.find_next_match_sync(String::from("X💻X"), 10000);
        assert_eq!(format!("{:?}", result2), "None");
    }

    #[test]
    fn should_handle_regex_g() {
        let mut scanner = Scanner::new(vec![String::from("\\G-and")]);
        let result = scanner.find_next_match_sync(String::from("first-and-second"), 0);
        assert_eq!(format!("{:?}", result), "None");

        let result2 = scanner.find_next_match_sync(String::from("first-and-second"), 5).unwrap();
        assert_eq!(serde_json::to_string(&result2).unwrap(), String::from("{\"index\":0,\"capture_indices\":[{\"start\":5,\"end\":9,\"length\":4}]}"));
    }
}