pub struct JavaSima {}

impl JavaSima {}

#[cfg(test)]
mod tests {
    use crate::analyser::Analyser;
    use std::path::PathBuf;

    #[test]
    fn should_identify_java_hello_world() {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).to_path_buf();
        let lang = root_dir
            .clone()
            .parent()
            .unwrap()
            .join("fixtures")
            .join("projects")
            .join("java")
            .join("hello")
            .join("HelloWorld.java");

        let files = Analyser::ident_by_dir(&lang, false, false);
        let file = &files[0];
        println!("{:?}", file);
    }
}
