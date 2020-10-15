use crate::facet::jvm_facet::JvmFacet;
// use regex::Regex;

// lazy_static! {
//     static ref JAVA_TEST: Regex =
//         Regex::new(r"").unwrap();
// }

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct JavaFacet {
    pub jvm: JvmFacet,
    pub include_test: bool,
}

impl JavaFacet {
    pub fn is_test() {}
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    #[test]
    fn should_ident_test_java_file() {
        let regex = Regex::new(r".*(Tests|Test).java").unwrap();
        assert!(regex.is_match("src/test/java/com/phodal/scie/ScieTests.java"));

        assert_eq!(false, regex.is_match("Hello.java"));
    }
}
