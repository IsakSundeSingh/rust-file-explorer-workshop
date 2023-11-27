use clap::Parser;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Options {
    #[arg(long, default_value_t = 1)]
    min_depth: usize,

    #[arg(long, default_value_t = 1)]
    max_depth: usize,
}

fn main() {
    let options = Options::parse();

    for entry in WalkDir::new(".")
        .min_depth(options.min_depth)
        .max_depth(options.max_depth)
    {
        println!("{}", entry.unwrap().path().display());
    }
}
