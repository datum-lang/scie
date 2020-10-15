use scie_model::artifact::CodeFile;
use std::fs::File;
use std::io::Write;

pub struct BinGen {}

impl BinGen {
    pub fn code_files(files: Vec<CodeFile>, path: &str) {
        let encoded: Vec<u8> = bincode::serialize(&files).unwrap();
        let mut file = File::create(path).unwrap();
        match file.write_all(&*encoded) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::bin_gen::BinGen;
    use scie_model::artifact::CodeFile;

    #[test]
    fn should_build_code_gen_file() {
        let code_file = CodeFile {
            path: "path".to_string(),
            name: "name".to_string(),
            elements: vec![],
        };
        BinGen::code_files(vec![code_file], "demo.bin");
    }
}
