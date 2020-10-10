use std::path::PathBuf;
use std::fs::File;
use std::io::Read;

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
    pub fn read_code(lang_test_dir: &PathBuf) -> String {
        let mut file = File::open(lang_test_dir).unwrap();
        let mut code = String::new();
        file.read_to_string(&mut code).unwrap();
        code
    }
}