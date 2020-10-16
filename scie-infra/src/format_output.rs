pub struct FormatOutput {

}

impl FormatOutput {
    pub fn hashmap() {
        println!(
            "{0: <10} | {1: <10} | {2: <10} | {3: <10}",
            "total", "blanks", "comments", "code"
        );
        println!("{0: <10} | {1: <10} | {2: <10} | {3: <10}", 0, 0, 0, 0);
        println!("{0: <10} | {1: <10} | {2: <10} | {3: <10}", 77, 0, 3, 74);
        println!("{0: <10} | {1: <10} | {2: <10} | {3: <10}", 112, 0, 6, 106);
        println!(
            "{0: <10} | {1: <10} | {2: <10} | {3: <10}",
            460, 0, 10, 1371
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::format_output::FormatOutput;

    #[test]
    fn should_filter_gitignore_rules() {
        FormatOutput::hashmap();
    }
}

