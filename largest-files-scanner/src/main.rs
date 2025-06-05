use std::{env, fs::{self, metadata}, path::{Path, PathBuf}};

const KB: u64 = 1024;
const MB: u64 = 1024 * KB;
const GB: u64 = 1024 * MB;
const TB: u64 = 1024 * GB;

fn main() {
    // Collect arguments to a list
    let args: Vec<String> = env::args().collect();
    
    // Prompt correct input
    if args.len() != 2 {
        println!("Usage: largest-files-scanner <path>");
        return;
    }

    // Create a PathBuf from args
    let path = PathBuf::from(&args[1]);

    // Use recursive function to create list of files and sizes 
    let mut file_list = match recursive(&path) {
        Ok(files) => files,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    // Sort list 
    file_list.sort_by(|a, b| b.1.cmp(&a.1));

    // Print top 10 from list
    for (path, size) in file_list.iter().take(10) {
        println!("{:<10} {}", format_size(*size), path.display());
    }




}

fn recursive(path: &Path) -> Result<Vec<(PathBuf, u64)>, std::io::Error> {
    let meta = metadata(path)?;

    let mut result: Vec<(PathBuf, u64)> = Vec::new();

    if meta.is_file() {
        result.push((path.to_path_buf(), meta.len()));
    } else if meta.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let children = recursive(&entry.path())?;
            result.extend(children);
        }
    }
    
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