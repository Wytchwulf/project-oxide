# File Summary

## Overview
Produce a file type summary report.
Recursively scan a directory and:
- Group files by extension
- Count number of files per type
- Calculate total size per type
- Output sorted summary

## Tools
Filesystem & Metadata

    fs::read_dir(path)
    Reads the contents of a directory and returns an iterator of entries.

    fs::metadata(path)
    Retrieves metadata about a file or directory, such as size or type.

    .is_file()
    Returns true if the path points to a regular file.

    .is_dir()
    Returns true if the path points to a directory.

    .len()
    Returns the size of a file in bytes (u64).

    .extension()
    Gets the file extension as an Option<OsStr>.

    .and_then(|e| e.to_str())
    Converts the OsStr into a &str safely. Returns None if it’s not valid UTF-8.

    .unwrap_or("")
    Provides a fallback empty string if the extension is missing or invalid.

    .to_lowercase()
    Converts the file extension string to lowercase for consistent grouping.

Path Handling

    Path
    A borrowed, read-only reference to a file system path.

    PathBuf
    An owned, mutable version of a path. Needed when storing paths.

    .extension()
    Gets the extension part of the file name.

    .display()
    Converts a Path or PathBuf into a printable form.

HashMap

    HashMap<String, (u64, usize)>
    Maps file extensions to a tuple of (total size in bytes, file count).

    .entry(ext).or_insert((0, 0))
    Gets a mutable reference to the value for ext, inserting (0, 0) if the key doesn't exist.

    entry.0 += size
    Adds to the total size for that file extension.

    entry.1 += 1
    Increments the file count for that extension.

    .iter()
    Creates an iterator over key-value pairs in the map.

    .collect()
    Converts an iterator into a collection, such as a Vec.

Sorting

    .sort_by(|a, b| b.1.0.cmp(&a.1.0))
    Sorts file types in descending order by total size.

    .take(n)
    Limits the output to the top n entries.

Formatting

    format!("{:.2} {}", value, unit)
    Formats a floating-point number to 2 decimal places followed by a unit (e.g., 12.34 MB).

    format!("{:<8} {:>6} files {:>10}", ...)
    Aligns the extension, count, and size for readable output.

