use std::{fs::DirEntry, io::Write, path::PathBuf};

use serde::Deserialize;

pub fn workspace_dir() -> std::path::PathBuf {
    let output = std::process::Command::new(env!("CARGO"))
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()
        .unwrap()
        .stdout;
    let cargo_path = std::path::Path::new(std::str::from_utf8(&output).unwrap().trim());
    cargo_path.parent().unwrap().to_path_buf()
}

pub fn cargo_home_dir() -> std::path::PathBuf {
    let cargo_home_output = std::process::Command::new("sh")
        .arg("-c")
        .arg("echo $CARGO_HOME")
        .output()
        .expect("Failed to execute command")
        .stdout;

    let cargo_home = std::str::from_utf8(&cargo_home_output).unwrap().trim();
    if cargo_home.is_empty() {
        let home_output = std::process::Command::new("sh")
            .arg("-c")
            .arg("echo $HOME")
            .output()
            .expect("Failed to execute command")
            .stdout;
        let home = std::str::from_utf8(&home_output).unwrap().trim();

        std::path::Path::new(&format!("{home}/.cargo")).to_path_buf()
    } else {
        std::path::Path::new(cargo_home).to_path_buf()
    }
}

#[derive(Deserialize, Debug)]
pub struct CratesIoCallRequest {
    #[serde(rename = "crate")]
    pub crate_map: CratesIoCall,
}

#[derive(Deserialize, Debug)]
pub struct CratesIoCall {
    pub repository: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GithubApiFileTree {
    pub tree: Vec<GithubApiFileTreeRow>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GithubApiFileTreeRow {
    pub path: String,
}

#[derive(Debug, Clone)]
pub enum StructOrEnum {
    Struct,
    Enum,
}

impl StructOrEnum {
    pub fn new_from_line(line: &str) -> Self {
        if line.trim_start().contains("struct ") || line.trim_start().contains("pub struct ") {
            StructOrEnum::Struct
        } else if line.trim_start().contains("enum ") || line.trim_start().contains("pub enum ") {
            StructOrEnum::Enum
        } else {
            panic!("Expected 'Struct' or 'Enum'")
        }
    }
}

pub fn write_to_file_cache(path: &str, text: &str) {
    let mut file = std::fs::File::create(path).expect(&format!("Failed to open file in file cache: {}", path));

    file.write_all(text.as_bytes())
        .expect(&format!("Failed to write file in file cache: {}", path));
}

pub fn get_all_files(dir: &DirEntry, paths: &mut Vec<PathBuf>) {
    let dir_values = std::fs::read_dir(dir.path())
        .expect(&format!("Could not read dir for path {:?}", dir.path().to_str()))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    for path in dir_values {
        if path.path().is_dir() {
            get_all_files(&path, paths)
        } else {
            if let Some(ext) = path.path().extension() {
                if let Some(e) = ext.to_str() {
                    if e == "rs" {
                        paths.push(path.path())
                    }
                }
            }
        }
    }
}
