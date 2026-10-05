use std::{
    fs::{self, File}, io::{self, Read, Write}, path::Path
};

use walkdir::WalkDir;
use zip::{ZipArchive, ZipWriter, write::SimpleFileOptions};

use crate::error::AppResult;

pub fn create(file_name: &str, folder: &str) -> AppResult<()> {
    let file = File::create(file_name)?;
    let mut zip = ZipWriter::new(file);

    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    let base_path = Path::new(folder);
    let walkdir = WalkDir::new(base_path);
    let entries = walkdir.into_iter();
    let mut buffer = Vec::new();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        let name = path.strip_prefix(base_path)?;
        if name.as_os_str().is_empty() {
            continue;
        }

        let name_str = name.to_string_lossy().replace('\\', "/");

        if path.is_dir() {
            println!("Add dir {}", name_str);
            zip.add_directory(format!("{}/", name_str), options)?;
        } else {
            println!("Add file {}", name_str);
            zip.start_file(name_str, options)?;

            let mut f = File::open(path)?;
            buffer.clear();
            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
        }
    }
    zip.finish()?;

    Ok(())
}

pub fn list(zip_file: &str) -> AppResult<()> {
    let file = File::open(zip_file)?;
    let mut archive = ZipArchive::new(file)?;
    for i in 0..archive.len() {
        let file_zip = archive.by_index(i)?;
        if file_zip.is_dir() {
            println!("[DIR] {}", file_zip.name());
        } else {
            println!("[FILE] {} {}", file_zip.name(), file_zip.size());
        }
    }

    Ok(())
}

pub fn extract(zip_file: &str, extract_folder: &str) -> AppResult<()> {
    let file = File::open(zip_file)?;
    let mut archive = ZipArchive::new(file)?;

    let target_dir = Path::new(extract_folder);
    for i in 0..archive.len() {
        let mut file_zip = archive.by_index(i)?;
        let outpath = match file_zip.enclosed_name() {
            Some(path) => target_dir.join(path),
            None => continue,
        };

        if file_zip.is_dir() {
            println!("[DIR] {}", file_zip.name());
            fs::create_dir_all(&outpath)?;

        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)?;
                }
            }
            // println!("[FILE] {} {}", file_zip.name(), file_zip.size());
            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file_zip, &mut outfile)?;
            println!("Распакован: {:?}", outpath);
        }
    }
    Ok(())
}
