use onig::Regex;


#[derive(Debug, Clone)]
pub struct IOnigCaptureIndex {
    pub start: i32,
    pub end: i32,
    pub length: i32,
}

#[derive(Debug, Clone)]
pub struct IOnigMatch {
    pub index: i32,
    pub capture_indices: Vec<IOnigCaptureIndex>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Scanner {
    pub index: i32,
    pub patterns: Vec<String>,
}

impl Scanner {
    pub fn new(patterns: Vec<String>) -> Self {
        Scanner {
            index: 0,
            patterns,
        }
    }

    pub fn find_next_match_sync(self, str: String, start_position: i32) {
        if self.index >= self.patterns.clone().len() as i32 {}

        for pattern in self.patterns {
            let regex = Regex::new(pattern.as_str()).unwrap();
            for (i, pos) in regex.captures(str.as_str()).unwrap().iter_pos().enumerate() {
                match pos {
                    Some((beg, end)) =>
                        println!("Group {} captured in position {}:{}", i, beg, end),
                    None =>
                        println!("Group {} is not captured", i)
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::scanner::scanner::Scanner;

    #[test]
    fn should_handle_simple_regex() {
        let regex = vec![String::from("ell"), String::from("wo")];
        let scanner = Scanner::new(regex);
        scanner.find_next_match_sync(String::from("Hello world!"), 0);
    }
}