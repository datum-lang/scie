use walkdir::WalkDir;

pub fn walk_dir(path: String) {
    for entry in WalkDir::new("foo") {
        let entry = entry.unwrap();
        println!("{}", entry.path().display());
    }
}

fn main() {
    println!("Hello, world!");
    walk_dir("");
}

#[cfg(test)]
mod tests {

}
