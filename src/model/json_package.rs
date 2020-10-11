use crate::model::contribute::Contribute;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonPackage {
    name: String,
    contributes: Contribute
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::finder::Finder;
    use crate::model::json_package::JsonPackage;

    #[test]
    fn should_parse_json_package_optional() {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).to_path_buf();
        let config = root_dir.clone()
            .join("extensions").join("java").join("package.json");

        let code = Finder::read_code(&config);
        let pkg: JsonPackage = serde_json::from_str(&code).unwrap();

        println!("{:?}", pkg);

        assert_eq!("java", pkg.name);
        assert_eq!(1, pkg.contributes.languages.unwrap().len());
    }
}