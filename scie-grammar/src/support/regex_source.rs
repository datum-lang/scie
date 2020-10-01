use regex::{Regex, Captures};
use scie_scanner::scanner::scie_scanner::IOnigCaptureIndex;

pub struct RegexSource {}

impl RegexSource {
    pub fn has_captures(regex_source: Option<String>) -> bool {
        let capturing_regex_source = r"\$(\d+)|\$\{(\d+):/(downcase|upcase)\}";

        match regex_source {
            None => false,
            Some(source) => {
                let re = Regex::new(capturing_regex_source).unwrap();
                re.is_match(source.as_str())
            }
        }
    }

    pub fn replace_captures(
        regex_source: String,
        capture_source: String,
        capture_indices: Vec<IOnigCaptureIndex>,
    ) -> String {
        //  origin match strings: r"\$(\d+)|\$\{(\d+):/(downcase|upcase)\}";
        let expr2 = r"\$(?P<index>\d+)|\$\{(?P<commandIndex>\d+):/(?P<command>downcase|upcase)\}";
        let re = Regex::new(expr2).unwrap();

        let res = re.replace_all(&*regex_source, |capts: &Captures| {
            let capture_str;
            if capts.name("index").is_none() {
                capture_str = &capts["commandIndex"];
            } else {
                capture_str = &capts["index"];
            }
            let capture_index = (capture_str).parse::<usize>().unwrap();
            if capture_index > capture_indices.len() {
                return regex_source.clone();
            }

            let capture: IOnigCaptureIndex = capture_indices[capture_index].clone();
            let mut result = &capture_source[capture.start..capture.end];
            while result.as_bytes()[0] as char == '.' {
                result = &result.clone()[1..result.len()];
            }

            let command;
            if capts.name("command").is_none() {
                return String::from(result);
            }

             match &capts["command"] {
                "downcase" => {
                    command = result.to_uppercase();
                }
                "lowcase" => {
                    command = result.to_lowercase();
                }
                _ => {
                    command = String::from(result);
                }
            };

            return command;
        });

        return String::from(res);
    }
}

#[cfg(test)]
mod tests {
    use crate::support::regex_source::RegexSource;
    use scie_scanner::scanner::scie_scanner::IOnigCaptureIndex;

    #[test]
    fn should_replace_captures_for_upcase() {
        let source = String::from("support.function.target.$1.makefile");
        let capture_source = String::from(".SUFFIXES");

        let mut capture_indices = vec![];
        capture_indices.push(IOnigCaptureIndex { start: 0, end: 9, length: 9 });
        capture_indices.push(IOnigCaptureIndex { start: 0, end: 9, length: 9 });
        capture_indices.push(IOnigCaptureIndex { start: 1, end: 9, length: 8 });

        let string = RegexSource::replace_captures(source, capture_source, capture_indices);
        assert_eq!("support.function.target.SUFFIXES.makefile", string);
    }

    #[test]
    fn should_return_true_when_has_captures() {
        let captures =
            RegexSource::has_captures(Some(String::from("support.function.$1.makefile")));
        assert!(captures);
    }

    #[test]
    fn should_return_true_when_has_downcase() {
        let captures =
            RegexSource::has_captures(Some(String::from("storage.type.class.${1:/downcase}")));
        assert!(captures);
    }
}
