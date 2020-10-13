use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use walkdir::WalkDir;

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

    pub fn get_files(dir: &PathBuf, gitignore_path: Option<&PathBuf>) -> Vec<PathBuf> {
        let walk_dir = WalkDir::new(dir);

        let mut files = vec![];
        match gitignore_path {
            None => {
                for entry in walk_dir.into_iter() {
                    if entry.is_err() {
                        continue;
                    };

                    files.push(entry.unwrap().path().to_path_buf());
                }
            }
            Some(gitignore) => {
                let file = gitignore::File::new(&gitignore).unwrap();
                for entry in walk_dir.into_iter() {
                    if entry.is_err() {
                        continue;
                    };
                    let path = entry.unwrap().path().to_path_buf();
                    if !file.is_excluded(&*path).unwrap() {
                        files.push(path);
                    }
                }
            }
        }

        return files;
    }
}

#[cfg(test)]
mod tests {
    use crate::finder::Finder;
    use std::path::PathBuf;

    #[test]
    fn should_filter_gitignore_rules() {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();
        let code_dir = root_dir
            .clone()
            .join("fixtures")
            .join("finder")
            .join("ignore-test");

        let ignore_path = code_dir.clone().join(".gitignore");

        let files = Finder::get_files(&code_dir, Some(&ignore_path));
        assert_eq!(1, files.len())
    }
}
