use ignore::Walk;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use tempfile::tempdir;
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
    pub fn read_code(file_path: &PathBuf) -> Result<String, String> {
        let result = File::open(file_path);
        return match result {
            Ok(mut file) => {
                let mut buffer: Vec<u8> = Vec::new();
                let _ = file.read_to_end(&mut buffer);

                match String::from_utf8(buffer) {
                    Ok(str) => return Ok(str),
                    Err(err) => {
                        return Err(err.to_string());
                    }
                };
            }
            Err(err) => {
                println!("error: {:?}", file_path);
                return Err(err.to_string());
            }
        };
    }

    pub fn walk_filter_files(dir: &PathBuf) -> Vec<PathBuf> {
        let mut files = vec![];
        for result in Walk::new(dir) {
            match result {
                Ok(entry) => {
                    files.push(entry.path().to_path_buf());
                }
                Err(err) => println!("ERROR: {}", err),
            }
        }

        return files;
    }

    pub fn get_filter_files(dir: &PathBuf, gitignore_path: Option<&PathBuf>) -> Vec<PathBuf> {
        let walk_dir = WalkDir::new(dir);

        let dir = tempdir().unwrap();
        let genearte_ignore_file_path = dir.path().join("scie-ignore.txt");
        println!("generate ignore file: {:?}", genearte_ignore_file_path);

        let mut tmpfile = File::create(genearte_ignore_file_path.clone()).unwrap();
        let content: String = DEFAULT_VCS_EXCLUDES.join("\n");

        match gitignore_path {
            None => {
                write!(tmpfile, "{}", content).unwrap();
            }
            Some(ignore) => {
                let code = Finder::read_code(ignore).unwrap();
                write!(tmpfile, "{}", code).unwrap();
                write!(tmpfile, "{}", content).unwrap();
            }
        };

        let mut files = vec![];
        let file = gitignore::File::new(&genearte_ignore_file_path).unwrap();
        for entry in walk_dir.into_iter() {
            if entry.is_err() {
                continue;
            };
            let path = entry.unwrap().path().to_path_buf();
            if !file.is_excluded(&*path).unwrap() {
                files.push(path);
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

        let files = Finder::get_filter_files(&code_dir, Some(&ignore_path));
        assert_eq!(0, files.len())
    }
}
