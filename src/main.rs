use std::{fmt::Display, fs::Metadata, path::PathBuf, time::SystemTime};

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

    #[arg(long, default_value_t = false)]
    modified: bool,
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
        let modified = if options.modified {
            format!(
                "{:>25}",
                format!("\t{}\t", "Modified at".bold().underline())
            )
        } else {
            "".into()
        };

        println!(
            "{:>5}{}{}\t{:>11}{}",
            "",
            "Size".bold().underline(),
            modified,
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
        let metadata = entry.metadata().context(format!(
            "Failed extracting metadata for {}. Perhaps you are missing permissions?",
            entry.path().display()
        ))?;
        let modified_at = metadata
            .modified()
            .context("Could not get date modified for the entry")?;
        let size = FormatSize(metadata);
        let formatted_entry = FormatEntry(&entry);

        let formatted_date = if options.modified {
            let modified = FormatModifiedAt(modified_at);
            format!("{:>25}\t", modified)
        } else {
            "".into()
        };
        println!("{}\t{}{}", size, formatted_date, formatted_entry);
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

        const INDENTATION_SEQUENCE: &str = "⤷ ";

        let indent = INDENTATION_SEQUENCE
            .repeat(self.0.depth().saturating_sub(1))
            .dimmed();

        f.write_fmt(format_args!("{indent}{formatted_entry}"))
    }
}

struct FormatSize(Metadata);

impl Display for FormatSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:>9}", ByteSize(self.0.len())).green())
    }
}

struct FormatModifiedAt(SystemTime);

impl Display for FormatModifiedAt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let date = chrono::DateTime::<chrono::Utc>::from(self.0);
        f.write_fmt(format_args!(
            "{}",
            date.to_rfc2822()
                // SAFETY: We know it is in UTC so the stripping always works, probably 🤠
                .strip_suffix(" +0000")
                .unwrap()
                .blue()
        ))
    }
}
