use std::{
    fs::{metadata, read_dir},
    path::{Path, PathBuf},
};

const KB: u64 = 1024;
const MB: u64 = KB * 1024;
const GB: u64 = MB * 1024;
const TB: u64 = GB * 1024;

use clap::Parser;

#[derive(Parser)]
#[command(
    name = "Large File Finder",
    about = "Recursively scan a directory and find 'n' largest files"
)]
struct Args {
    path: String,
    #[arg(default_value_t = 10)]
    n: usize,
}

fn main() {
    let args = Args::parse();

    let path = PathBuf::from(args.path);

    let mut list = match recursive(&path) {
        Ok(result) => result,
        Err(e) => {
            eprint!("{}", e);
            return;
        }
    };

    list.sort_by(|a, b| b.1.cmp(&a.1));

    for (index, (path, size)) in list.iter().take(args.n).enumerate() {
        println!("{}. {:<10} {:>8}", index + 1, path.display(), format_size(*size));
    }
}

fn recursive(path: &Path) -> Result<Vec<(PathBuf, u64)>, std::io::Error> {
    let meta = metadata(path)?;
    let mut result: Vec<(PathBuf, u64)> = Vec::new();

    if meta.is_file() {
        result.push((path.to_path_buf(), meta.len()));
    };

    if meta.is_dir() {
        for entry in read_dir(path)? {
            let entry = entry?;
            let children = recursive(&entry.path())?;
            result.extend(children);
        }
    };

    Ok(result)
}

fn format_size(bytes: u64) -> String {
    for (unit, factor) in [("TB", TB), ("GB", GB), ("MB", MB), ("KB", KB)] {
        if bytes >= factor {
            return format!("{:.2} {}", bytes as f64 / factor as f64, unit);
        }
    }

    format!("{:.2} B", bytes as f64)
}
