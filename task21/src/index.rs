use std::fs::File;
use std::io::BufReader;
use std::io::{BufRead, Result, Write};

use serde::Serialize;

#[derive(Serialize)]
pub struct IndexFile {
    pub file_path: String,
    pub hash: String,
}

pub const PATH_INDEX: &str = ".mygit/index";
pub fn add_to_index(path: &str, hash: &str) -> Result<()> {
    let mut file_index = File::options().append(true).open(PATH_INDEX)?;
    writeln!(file_index, "{} {}", hash, path)?;
    Ok(())
}

pub fn read_index() -> Result<Vec<IndexFile>> {
    let file_index = File::open(PATH_INDEX)?;
    let mut vec_files = Vec::<IndexFile>::new();
    let buf_read = BufReader::new(file_index);
    for line in buf_read.lines() {
        let line = line?;
        if let Some((hash, file_path)) = line.split_once(" ") {
            vec_files.push(IndexFile {
                file_path: file_path.to_string(),
                hash: hash.to_string(),
            });
        } else {
            eprintln!("Некорректный формат строки: {}", line);
        }
    }
    Ok(vec_files)
}

