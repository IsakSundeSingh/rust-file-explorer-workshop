use std::{fmt::Display, fs::Metadata, path::PathBuf};

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
        let size = FormatSize::try_from(&entry)?;
        let formatted_entry = FormatEntry(&entry);
        println!("{}\t{}", size, formatted_entry);
    }

    Ok(())
}

struct FormatEntry<'walk_dir_loop>(&'walk_dir_loop DirEntry);

impl<'walk_dir_loop> Display for FormatEntry<'walk_dir_loop> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path = self.0.path();
        let formatted_entry = if path.is_file() {
            path.display().to_string().white()
        } else if path.is_dir() {
            path.display().to_string().blue()
        } else {
            // We'll assume symlinks
            path.display().to_string().yellow()
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

impl<'walk_dir_loop> TryFrom<&'walk_dir_loop DirEntry> for FormatSize {
    type Error = anyhow::Error;

    fn try_from(entry: &DirEntry) -> Result<Self, Self::Error> {
        Ok(Self(entry.metadata()?))
    }
}
