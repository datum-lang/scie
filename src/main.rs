use scie_grammar::inter::{IRawCaptures, ILocatable, ILocation};
use std::collections::HashMap;

fn main() {
    let location = ILocation {
        filename: "".to_string(),
        line: "".to_string(),
        chart: "".to_string()
    };
    println!("{:?}", location)
}
