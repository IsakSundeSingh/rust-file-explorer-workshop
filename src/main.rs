use std::{fmt::Display, fs::Metadata, path::PathBuf};

use anyhow::Context;
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
        let entry = entry.context("Error getting file entry")?;
        let size = FormatSize(entry.metadata().context(format!(
            "Failed extracting metadata for {}. Perhaps you are missing permissions?",
            entry.path().display()
        ))?);
        let formatted_entry = FormatEntry(&entry);
        println!("{}\t{}", size, formatted_entry);
    }

    Ok(())
}

struct FormatEntry<'walk_dir_loop>(&'walk_dir_loop DirEntry);

impl<'walk_dir_loop> Display for FormatEntry<'walk_dir_loop> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path = self.0.path();
        // SAFETY: We can safely unwrap here as we know the path contains at least one part (e.g. `.` or `./thing`, or so on)
        let name = path.iter().last().unwrap().to_string_lossy();
        let formatted_entry = if path.is_file() {
            name.white()
        } else if path.is_dir() {
            name.blue()
        } else {
            // We'll assume symlinks
            name.yellow()
        };
        f.write_fmt(format_args!("{formatted_entry:>15}"))
    }
}

struct FormatSize(Metadata);

impl Display for FormatSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:>9}", ByteSize(self.0.len())).green())
    }
}
