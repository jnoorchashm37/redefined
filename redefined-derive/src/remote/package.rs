use std::io::BufRead;

use super::types::CratesIoCallRequest;
use crate::remote::types::{workspace_dir, GithubApiFileTree};

/// parsing of the package from crates-io OR github
#[derive(Debug, Clone)]
pub struct Package {
    pub root_url: String,
    pub version:  String,
    pub kind:     PackageKind,
}

impl Package {
    pub fn new(package: String) -> std::io::Result<Self> {
        let root_path = workspace_dir();
        let path = root_path.to_str().unwrap();

        let cargo_lock_file_path = format!("{path}/Cargo.lock");

        let file = std::fs::File::open(cargo_lock_file_path)?;
        let reader = std::io::BufReader::new(file).lines();

        let searching_for = format!("name = \"{package}\"");

        let mut version = None;

        let mut found = false;
        for line_res in reader {
            if let Ok(line) = line_res {
                if line == searching_for {
                    found = true;
                }

                if found && line.starts_with("version = ") {
                    version = Some(line.replace("version = ", ""));
                }

                if found && line.starts_with("source = ") {
                    let (url, kind) = if line.contains("https://github.com/rust-lang/crates.io-index") {
                        (Some(Default::default()), PackageKind::CratesIo(format!("https://crates.io/api/v1/crates/{package}")))
                    } else {
                        (Some(line.replace("source = ", "").replace("git+", "")), PackageKind::Github)
                    };

                    return Ok(Package {
                        root_url: url
                            .expect(&format!("could not parse 'source' for package '{package}' in cargo lock"))
                            .trim_matches('\"')
                            .to_string(),
                        version: version
                            .expect(&format!("could not parse 'version' for package '{package}' in cargo lock"))
                            .trim_matches('\"')
                            .to_string(),
                        kind,
                    })
                }
            }
        }

        panic!("Cound Not Parse Package: '{package}'")
    }

    pub async fn get_registry_url(&mut self, web_client: &reqwest::Client) -> reqwest::Result<String> {
        let url = self.kind.crates_io_registry_url();

        let crates_io_text = web_client
            .get(&url)
            .header("User-Agent", "request")
            .send()
            .await?
            .text()
            .await
            .expect("Could not deserialize crates-io kind to text");

        let crates_io: CratesIoCallRequest =
            serde_json::from_str(&crates_io_text).expect(&format!("Could not deserialize crates-io kind for url: {}\ntext: {}", url, crates_io_text));

        panic!("CALL: {crates_io}");

        Ok(crates_io.crate_map.repository)
    }
}

#[derive(Debug, Clone)]
pub enum PackageKind {
    CratesIo(String),
    Github,
}

impl PackageKind {
    pub fn crates_io_registry_url(&self) -> String {
        match self {
            PackageKind::CratesIo(url) => url.clone(),
            _ => unreachable!("Cannot be github"),
        }
    }

    pub fn is_crates_io(&self) -> bool {
        match self {
            PackageKind::CratesIo(_) => true,
            PackageKind::Github => false,
        }
    }
}

/// retrieves github urls from crates-io
#[derive(Debug, Clone)]
pub struct GithubApiUrls {
    pub root_url:          String,
    pub file_tree_url:     String,
    pub base_contents_url: String,
    pub commit:            String,
    pub is_crates_io:      bool,
}

impl GithubApiUrls {
    pub async fn get_all_urls(&self, web_client: &reqwest::Client) -> reqwest::Result<Vec<(String, String)>> {
        let tree_text = web_client
            .get(&self.file_tree_url)
            .header("User-Agent", "request")
            .send()
            .await?
            .text()
            .await
            .expect("Could not deserialize all url path results to text");

        let tree: GithubApiFileTree = serde_json::from_str(&tree_text).expect(&format!(
            "Could not deserialize all text of all url path results from file {} to text to GithubApiFileTree: {tree_text}",
            self.file_tree_url
        ));

        let all_paths = tree
            .tree
            .into_iter()
            .map(|path| path.path)
            .filter(|p| p.ends_with(".rs"))
            .map(|path| (format!("{}{path}", self.base_contents_url), path.replace("/", "_")))
            .collect();

        Ok(all_paths)
    }
}
