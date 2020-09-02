use regex::Regex;

pub struct RegexSource {}


impl RegexSource {
    pub fn has_captures(regex_source: Option<String>) -> bool {
        let capturing_regex_source = r"\$(\d+)|\$\{(\d+):/(downcase|upcase)\}";

        match regex_source {
            None => {
                false
            },
            Some(source) => {
                let re = Regex::new(capturing_regex_source).unwrap();
                re.is_match(source.as_str())
            },
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::support::regex_source::RegexSource;

    #[test]
    fn should_return_true_when_has_captures() {
        let captures = RegexSource::has_captures(Some(String::from("support.function.$1.makefile")));
        assert!(captures);
    }

    #[test]
    fn should_return_true_when_has_downcase() {
        let captures = RegexSource::has_captures(Some(String::from("storage.type.class.${1:/downcase}")));
        assert!(captures);
    }
}
