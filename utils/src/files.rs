use std::{fs, io};

/// Reads in a file with the given `filepath`.
pub fn read_file_string(filepath: &str) -> io::Result<String> {
    fs::read_to_string(filepath)
}
