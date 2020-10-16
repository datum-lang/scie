use crate::validate::Validate;
use clap::Clap;
use scie_bingen::bin_gen::BinGen;
use scie_core::analyser::Analyser;
use std::path::Path;

pub mod validate;

#[derive(Clap)]
#[clap(version = "0.1", author = "Phodal HUANG<h@phodal.com>")]
struct Opts {
    #[clap(short, long, default_value = "default.conf")]
    _config: String,
    #[clap(long)]
    json: bool,
    #[clap(long, short)]
    debug: bool,
    #[clap(short, long, default_value = ".")]
    path: String,
    #[clap(short, long, parse(from_occurrences))]
    _verbose: i32,
}

fn main() {
    let opts: Opts = Opts::parse();

    print!("\x1B[2J\x1B[1;1H"); //clear screen and move to first line
    // println!("Value for config: {}", opts.config);
    println!("Using input file: {}", opts.path);

    if !Validate::is_valid_path(opts.path.clone()) {
        println!("is invalid path: {:?}", opts.path.clone());
        return;
    }

    let path = Path::new(&opts.path);
    let files = Analyser::ident_by_dir(&path.to_path_buf(), opts.debug, true);
    if opts.json {
        BinGen::jsonify(files.clone(), "demo.json");
    } else {
        BinGen::code_files(files, "demo.bin");
    }
}

#[cfg(test)]
mod tests {
    use std::io::{stdout, Write};

    use crossterm::{
        execute,
        style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
        ExecutableCommand, Result,
    };

    fn sample_term() -> Result<()> {
        execute!(
        stdout(),
        SetForegroundColor(Color::Blue),
        SetBackgroundColor(Color::Red),
        Print("Styled text here."),
        ResetColor
    )?;

        // or using functions
        stdout()
            .execute(SetForegroundColor(Color::Blue))?
            .execute(SetBackgroundColor(Color::Red))?
            .execute(Print("Styled text here."))?
            .execute(ResetColor)?;

        Ok(())
    }

    #[test]
    fn should_run() {
        let _result = sample_term();
        assert_eq!(1, 1)
    }
}