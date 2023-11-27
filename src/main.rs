use std::path::PathBuf;

use bytesize::ByteSize;
use clap::Parser;
use colored::Colorize;
use walkdir::{DirEntry, WalkDir};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Options {
    #[arg(short, long)]
    path: Option<PathBuf>,

    #[arg(long, default_value_t = 1)]
    min_depth: usize,

    #[arg(long, default_value_t = 1)]
    max_depth: usize,

    #[arg(long, default_value_t = false)]
    headers: bool,

    #[arg(long, default_value_t = false)]
    hidden: bool,
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

fn main() -> anyhow::Result<()> {
    let options = Options::parse();

    if options.headers {
        println!(
            "{:>5}{}\t{:>11}{}",
            "",
            "Size".bold().underline(),
            "",
            "Name".bold().underline()
        );
    }

    for entry in WalkDir::new(options.path.unwrap_or(".".into()))
        .min_depth(options.min_depth)
        .max_depth(options.max_depth)
        .into_iter()
        .filter_entry(|entry| options.hidden || !is_hidden(entry))
    {
        let entry = entry?;
        let path = entry.path();
        let formatted_entry = if path.is_file() {
            path.display().to_string().white()
        } else if path.is_dir() {
            path.display().to_string().blue()
        } else {
            // We'll assume symlinks
            path.display().to_string().yellow()
        };

        let size = ByteSize(entry.metadata()?.len());

        println!("{:>9}\t{:>15}", size, formatted_entry);
    }

    Ok(())
}
