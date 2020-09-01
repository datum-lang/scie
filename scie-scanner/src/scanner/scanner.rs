use onig::{Regex, EncodedBytes, SearchOptions, MatchParam, Error};


#[derive(Debug, Clone)]
pub struct IOnigCaptureIndex {
    pub start: usize,
    pub end: usize,
    pub length: usize,
}

#[derive(Debug, Clone)]
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

    pub fn find_next_match_sync(&mut self, str: String, start_position: i32) -> IOnigMatch {
        if self.index >= self.patterns.clone().len() {}

        let pattern = self.patterns[self.index].clone();

        let regex = Regex::new(pattern.as_str()).unwrap();
        let mut capture_indices = vec![];
        for (i, pos) in regex.captures(str.as_str()).unwrap().iter_pos().enumerate() {
            match pos {
                Some((start, end)) => {
                    let capture = IOnigCaptureIndex {
                        start,
                        end,
                        length: end - start
                    };
                    capture_indices.push(capture)
                }
                None => {}
            }
        }

        let index = self.index.clone();

        self.index = self.index + 1;
        IOnigMatch {
            index,
            capture_indices,
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::scanner::scanner::Scanner;

    #[test]
    fn should_handle_simple_regex() {
        let regex = vec![String::from("ell"), String::from("wo")];
        let mut scanner = Scanner::new(regex);
        let s = String::from("Hello world!");
        let result = scanner.find_next_match_sync(s.clone(), 0);
        assert_eq!(result.index, 0);
        assert_eq!(result.capture_indices[0].start, 1);
        assert_eq!(result.capture_indices[0].end, 4);

        let second_result = scanner.find_next_match_sync(s, 2);
        assert_eq!(second_result.index, 1);
        assert_eq!(second_result.capture_indices[0].start, 6);
        assert_eq!(second_result.capture_indices[0].end, 8);
    }

    #[test]
    fn should_handle_simple2() {
        let regex = vec![String::from("a"), String::from("b"), String::from("c")];
        let mut scanner = Scanner::new(regex);
    }

}