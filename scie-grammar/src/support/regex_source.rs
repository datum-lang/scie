use regex::Regex;

pub struct RegexSource {}


impl RegexSource {
    pub fn has_captures(regex_source: Option<String>) -> bool {
        let capturing_regex_source = r"/\$(\d+)|\${(\d+):\/(downcase|upcase)}/";

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

