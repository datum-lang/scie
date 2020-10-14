use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.1", author = "Phodal HUANG<h@phodal.com>")]
struct Opts {
    #[clap(short, long, default_value = "default.conf")]
    config: String,
    #[clap(short, long, default_value = ".")]
    path: String,
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
}

fn main() {
    let opts: Opts = Opts::parse();

    // println!("Value for config: {}", opts.config);
    println!("Using input file: {}", opts.path);
}
