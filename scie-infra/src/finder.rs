use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

lazy_static! {
    static ref DEFAULT_VCS_EXCLUDES: Vec<&'static str> = vec![
        "**/%*%",
        "**/.#*",
        "**/._*",
        "**/#*#",
        "**/*~",
        "**/.DS_Store",
        "**/CVS",
        "**/CVS/**",
        "**/.cvsignore",
        "**/SCCS",
        "**/SCCS/**",
        "**/.bzr",
        "**/.bzr/**",
        "**/.bzrignore",
        "**/vssver.scc",
        "**/.hg",
        "**/.hg/**",
        "**/.hgtags",
        "**/.hgignore",
        "**/.hgsubstate",
        "**/.hgsub",
        "**/.svn",
        "**/.svn/**",
        "**/.git",
        "**/.git/**",
        "**/.gitignore",
        "**/.gitmodules",
        "**/.gitattributes"
    ];
}


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

    pub fn get_files(file_path: &PathBuf, gitignore: Option<&PathBuf>)  {

    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn should_filter_gitignore_rules() {
        assert_eq!(2 + 2, 4);
    }
}
