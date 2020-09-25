extern crate onig;

use onig::Regex;

fn main() {
    print_match_result("\\G", "\t$(CC) -o $@ $^ $(CFLAGS)\n");
    print_match_result("^\t", "\t$(CC) -o $@ $^ $(CFLAGS)\n");
}

fn print_match_result(pattern: &str, str: &str ) {
    let regex = Regex::new(pattern).unwrap();
    for (i, pos) in regex.captures(str).unwrap().iter_pos().enumerate() {
        match pos {
            Some((beg, end)) =>
                println!("Group {} captured in position {}:{}", i, beg, end),
            None =>
                println!("Group {} is not captured", i)
        }
    }
}
