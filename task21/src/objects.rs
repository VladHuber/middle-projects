use sha1::{Digest, Sha1};
use std::fs::{self, File};
use std::io::{BufReader, Read, Result, Write};
use std::path::PathBuf;

use crate::index::IndexFile;

const OBJECTS_DIR: &str = ".mygit/objects";

pub fn hash_file(file_dir: &str) -> Result<String> {
    let file = File::open(file_dir)?;
    let mut buff_read = BufReader::new(file);
    let mut sha = Sha1::new();
    let mut buffer = [0; 4096];

    loop {
        let count = buff_read.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        sha.update(&buffer[..count]);
    }
    let result = sha.finalize();

    Ok(hex::encode(result))
}

pub fn slice_hash(hash: String) -> (String, String) {
    let mut rest = hash;
    let prefix = rest.drain(0..2).collect();
    (prefix, rest)
}

pub fn save_object(file: &IndexFile) -> Result<()> {
    let (prefix, rest) = slice_hash(file.hash.clone());
    let content = fs::read(&file.file_path)?;
    let mut dir_hash = PathBuf::from(OBJECTS_DIR);
    dir_hash.push(prefix);

    fs::create_dir_all(&dir_hash)?;

    dir_hash.push(rest);
    let mut file_hash = File::create(dir_hash)?;
    file_hash.write_all(&content)?;
    Ok(())
}

pub fn save_tree(index_files: &Vec<IndexFile>) -> Result<String> {
    let content = bincode::serialize(index_files).unwrap();

    let mut hasher = Sha1::new();
    hasher.update(&content);
    let hash = hex::encode(hasher.finalize());

    let (prefix, rest) = slice_hash(hash.clone());

    let mut object_path = PathBuf::from(OBJECTS_DIR);
    object_path.push(prefix);

    fs::create_dir_all(&object_path)?;

    object_path.push(rest);

    let mut file = File::create(object_path)?;
    file.write_all(&content)?;

    Ok(hash)
}

pub fn save_commit(hash_tree: String, message: &str) -> Result<String> {
    let mut content = format!("tree: {}\nmessage: {}", hash_tree, message);
    let head_hash = read_head_hash();
    if head_hash.trim() != ""{
        content.push_str(format!("\nparent: {}", head_hash).as_str());
    }
    let mut hasher = Sha1::new();
    hasher.update(&content);
    let hash_commit = hex::encode(hasher.finalize());

    let (prefix, rest) = slice_hash(hash_commit.clone());

    let mut object_path = PathBuf::from(OBJECTS_DIR);
    object_path.push(prefix);

    fs::create_dir_all(&object_path)?;

    object_path.push(rest);

    let mut file = File::create(object_path)?;
    file.write_all(&content.as_bytes())?;

    Ok(hash_commit)
}

pub fn save_head(hash: String) -> Result<()> {
    const DEFAULT_HEAD_PATH: &str = ".mygit/refs/heads";
    let mut dir_head = PathBuf::from(DEFAULT_HEAD_PATH);
    fs::create_dir_all(&dir_head)?;
    dir_head.push("master");
    let mut file = File::create(dir_head)?;
    file.write_all(&hash.as_bytes())?;

    const DEFAULT_HEAD_PATH_CONTENT: &str = "ref: refs/heads/master";
    const HEAD_PATH: &str = ".mygit/HEAD";
    let mut file = File::create(HEAD_PATH)?;
    file.write_all(DEFAULT_HEAD_PATH_CONTENT.as_bytes())?;

    Ok(())
}

#[derive(Debug, Default,Clone)]
pub struct Commit {
    pub hash: String,
    pub tree: String,
    pub comment: String,
    pub parent: String,
}
pub fn check_commits() -> Result<Vec<Commit>> {
    let head_hash = read_head_hash();
    let mut commits = Vec::<Commit>::new();

    read_parent_commit(head_hash, &mut commits);

    Ok(commits)
}
fn read_parent_commit(hash:String, commits: &mut Vec<Commit>){
    let (prefix, rest) = slice_hash(hash.trim().to_string());
    let mut commit_dir = PathBuf::from(OBJECTS_DIR);
    commit_dir.push(prefix);
    commit_dir.push(rest);
    let commit_content = fs::read(commit_dir).unwrap();
    let commit_content = String::from_utf8(commit_content).unwrap();
    let mut commit = Commit::default();
    commit.hash = hash.trim().to_string();
    for line in commit_content.lines() {
        let (param, val) = line.split_once(":").unwrap();
        let val = val.trim();
        match param {
            "tree" => commit.tree = val.to_string(),
            "message" => commit.comment = val.to_string(),
            "parent" => commit.parent = val.to_string(),
            _ => {}
        }
    }
    commits.push(commit.clone());
    if !commit.parent.is_empty(){
        read_parent_commit(commit.parent, commits);
    }
}
fn read_head_hash() -> String {
    const HEAD_PATH: &str = ".mygit/refs/heads/master";
    let head_content = fs::read(HEAD_PATH).unwrap();
    let head_hash = String::from_utf8(head_content).unwrap();
    head_hash
}
