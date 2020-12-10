use crate::contribute::Contribute;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonPackage {
    pub name: String,
    pub contributes: Contribute,
}

#[cfg(test)]
mod tests {
    use crate::json_package::JsonPackage;
    use scie_infra::finder::Finder;
    use std::path::PathBuf;

    #[test]
    fn should_parse_json_package_optional() {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();
        let config = root_dir
            .clone()
            .join("extensions")
            .join("java")
            .join("package.json");

        let code = Finder::read_code(&config).unwrap();
        let pkg: JsonPackage = serde_json::from_str(&code).unwrap();

        assert_eq!("java", pkg.name);
        assert_eq!(1, pkg.contributes.languages.unwrap().len());
    }
}
