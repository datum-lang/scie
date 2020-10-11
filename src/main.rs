#[macro_use]
extern crate serde_derive;

extern crate serde;

#[macro_use]
extern crate lazy_static;

pub mod artifact;
pub mod bindata;
pub mod finder;
pub mod identify;

fn main() {}

#[cfg(test)]
mod tests {
    use crate::identify::Identify;
    use scie_infra::finder::Finder;
    use std::path::PathBuf;

    #[test]
    fn should_build_first_file() {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).to_path_buf();
        let lang = root_dir
            .clone()
            .join("extensions")
            .join("java")
            .join("syntaxes")
            .join("java.tmLanguage.json");

        let code_dir = root_dir
            .join("fixtures")
            .join("test-cases")
            .join("e2e")
            .join("java")
            .join("HelloWorld.java");
        let code = Finder::read_code(&code_dir);

        let elements = Identify::identify_file(lang, code);

        println!("{:?}", elements);
        assert_eq!(39, elements.len());
    }
}
