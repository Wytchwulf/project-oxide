use std::{
    collections::HashMap,
    env,
    fs::{self, metadata},
    path::{Path, PathBuf},
};

const KB: u64 = 1024;
const MB: u64 = KB * 1024;
const GB: u64 = MB * 1024;
const TB: u64 = GB * 1024;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        print!("Usage: file-summary <PATH>");
        return;
    };

    // Create path from argument
    let path = PathBuf::from(&args[1]);

    // Populate hashmap from recursive file walk
    let map = match recursive(&path) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    // Convert HashMap to List
    let mut summary: Vec<_> = map.iter().collect();
    // Sort list by file size -- Compare b with &a for Descending 
    summary.sort_by(|a, b| b.1.0.cmp(&a.1.0));

    // Print data for each entry in list
    for (ext, (size, count)) in summary {
        // {:<10} → left-align ext to 10 chars
        // {:>6}  → right-align count to 6 chars
        // {:>10} → right-align formatted size to 10 chars
        // *size  → dereference &u64 to pass u64 to format_size()
        println!("{:<10} {:>6} files  {:>10}", ext, count, format_size(*size));
    }
}

fn recursive(path: &Path) -> Result<HashMap<String, (u64, usize)>, std::io::Error> {
    // Access metadata of path
    let meta = metadata(path)?;
    // Create an empty HashMap to store (ext → (total size, count))
    let mut result: HashMap<String, (u64, usize)> = HashMap::new();

    // If path lands on a file
    if meta.is_file() {
        // Extract size of file in bytes
        let size = meta.len();

        let ext = path
            // Get extension
            .extension()
            // Convert OsStr to Str
            .and_then(|e| e.to_str())
            // Convert extension to lowercase
            .map(|s| s.to_lowercase())
            // Use extension if it exists; otherwise return "(none)" using a closure 
            .unwrap_or_else(|| "(none)".to_string());

        // Give me the value for ext, or insert (0, 0) if it’s not there — and give me a mutable reference to it.
        let entry = result.entry(ext).or_insert((0, 0));
        entry.0 += size;
        entry.1 += 1;
    };

    // If path lands on a directory
    if meta.is_dir() {
        // Read the contents of the directory and for each:
        for entry in fs::read_dir(path)? {
            // Give me the entry, not the errors
            let entry = entry?;
            // Create a hashmap and populate it with recursion
            let children = recursive(&entry.path())?;
            // Merge child summary into the current result map
            for (ext, (size, count)) in children {
                let entry = result.entry(ext).or_insert((0, 0));
                entry.0 += size;
                entry.1 += count;
            }
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
