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

#[derive(Deserialize, Debug)]
pub struct CratesIoCallRequest {
    #[serde(rename = "crate")]
    pub crate_map: CratesIoCall,
}

#[derive(Deserialize, Debug)]
pub struct CratesIoCall {
    pub homepage: String,
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
