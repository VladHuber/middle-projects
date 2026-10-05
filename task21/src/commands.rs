use std::fs;
use std::fs::File;
use std::io::Result;

use crate::index::{add_to_index, read_index};
use crate::objects::{check_commits, hash_file, save_commit, save_head, save_object, save_tree};

const OBJECTS_DIR: &str = ".mygit/objects";
pub fn init() -> Result<()> {
    fs::create_dir_all(OBJECTS_DIR)?;
    let create_files = vec![".mygit/index", ".mygit/HEAD"];
    for file in create_files {
        File::create(file)?;
    }
    Ok(())
}

pub fn add(file_dir: &str) -> Result<()> {
    let sha_code = hash_file(file_dir)?;
    add_to_index(file_dir, &sha_code)?;
    Ok(())
}

pub fn status() -> Result<()> {
    let index_files = read_index()?;
    for index_file in index_files.iter() {
        let current_hash = hash_file(&index_file.file_path)?;
        let status = {
            if current_hash == index_file.hash {
                "staged"
            } else {
                "modified"
            }
        };
        println!("{}: {}", status, index_file.file_path);
    }
    Ok(())
}

pub fn commit(message: &str) -> Result<()> {
    let index_files = read_index()?;

    for file in index_files.iter() {
        save_object(file)?;
    }
    let tree_hash = save_tree(&index_files)?;
    println!("tree: {}", tree_hash);
    let commit_hash = save_commit(tree_hash, message)?;
    println!("commit: {}", commit_hash);
    save_head(commit_hash)?;
    Ok(())
}

pub fn log() -> Result<()>{
    let commits = check_commits()?;
    for commit in commits{
        println!("{}\n{}\n", commit.hash, commit.comment);
    }

    Ok(())
}