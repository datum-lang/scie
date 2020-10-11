use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub struct Finder {}

impl Finder {
    pub fn read_code(file_path: &PathBuf) -> String {
        let result = File::open(file_path);
        match result {
            Ok(mut file) => {
                let mut code = String::new();
                file.read_to_string(&mut code).unwrap();
                code
            }
            Err(err) => {
                println!("{:?}", file_path);
                panic!(err.to_string())
            }
        }
    }
}
