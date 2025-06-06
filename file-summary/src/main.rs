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

    let path = PathBuf::from(&args[1]);

    let map = match recursive(&path) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let mut summary: Vec<_> = map.iter().collect();
    summary.sort_by(|a, b| b.1.0.cmp(&a.1.0));

    for (ext, (size, count)) in summary {
        println!("{:<8} {:>6} files  {:>10}", ext, count, format_size(*size));
    }
}

fn recursive(path: &Path) -> Result<HashMap<String, (u64, usize)>, std::io::Error> {
    let meta = metadata(path)?;
    let mut result: HashMap<String, (u64, usize)> = HashMap::new();

    if meta.is_file() {
        let size = meta.len();
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_lowercase())
            .unwrap_or_else(|| "(none)".to_string());

        let entry = result.entry(ext).or_insert((0, 0));
        entry.0 += size;
        entry.1 += 1;
    };

    if meta.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let children = recursive(&entry.path())?;
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
