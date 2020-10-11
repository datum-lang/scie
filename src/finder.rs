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
