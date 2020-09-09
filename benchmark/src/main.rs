use std::path::{Path, PathBuf};
use std::env;
use std::ffi::OsStr;

fn main() {
    println!("Hello, world!");
    let target_dir = get_target_dir();
    let root_dir = get_top_dir(&*target_dir);
    println!("{:?}", root_dir);
}

// https://github.com/rust-lang/cargo/issues/2841
fn get_target_dir() -> PathBuf {
    let bin = env::current_exe().expect("exe path");
    let mut target_dir = PathBuf::from(bin.parent().expect("bin parent"));
    while target_dir.file_name() != Some(OsStr::new("target")) {
        target_dir.pop();
    }
    target_dir
}

fn get_top_dir<'a>(target_dir: &'a Path) -> &'a Path {
    target_dir.parent().expect("target parent")
}
