use scie_grammar::inter::{ILocatable, ILocation, IRawCaptures};
use std::collections::HashMap;

fn main() {
    let location = ILocation {
        filename: "".to_string(),
        line: "".to_string(),
        chart: "".to_string(),
    };
    println!("{:?}", location)
}
