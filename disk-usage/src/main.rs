use std::{env, fs, path::Path};

const KB: u64 = 1024;
const MB: u64 = 1024 * KB;
const GB: u64 = 1024 * MB;
const TB: u64 = 1024 * GB;

fn main() {
    // Collect cli arguments in a list
    let args: Vec<String> = env::args().collect();

    // If user does not provide a single argument exit program and advise usage
    if args.len() != 2 {
        print!("Usage: disk-usage <path>");
        return;
    }

    // create a path from argument
    let path = Path::new(&args[1]);

    // Pattern match
    let total = match recursive(path) {
        Ok(total) => total,
        Err(_) => return,
    };

    let result = format_size(total);

    println!("{}", result);
}

fn recursive(path: &Path) -> Result<u64, std::io::Error> {
    // Get metadata for the given path (follows symlinks)
    let meta = fs::metadata(path)?;
    
    // If it's a file, return its size in bytes
    if meta.is_file() {
        return Ok(meta.len());
    }

    // Initialize total byte counter
    let mut total: u64 = 0;

    // If it's a directory, iterate over its contents
    if meta.is_dir() {
        for entry in fs::read_dir(path)? {
            // Each entry is a Result<DirEntry, Error>; propagate error with ?
            let entry = entry?;

            // Recurse into the entry's path to get its size
            let size = recursive(&entry.path())?;

            // Accumulate the size
            total += size;
        }
    }

    Ok(total)
}

fn format_size(bytes: u64) -> String {
    for (unit, factor) in [("TB", TB), ("GB", GB), ("MB", MB), ("KB", KB)] {
        if bytes >= factor {
            return format!("{:.2} {}", bytes as f64 / factor as f64, unit);
        }
    }

    format!("{:.2} B", bytes as f64)
}