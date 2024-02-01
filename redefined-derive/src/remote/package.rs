use std::io::BufRead;

use super::types::{CratesIoCallRequest, RemoteType};
use crate::remote::types::{workspace_dir, GithubApiFileTree};

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
                        (Some(Default::default()), PackageKind::CratesIo("https://github.com/rust-lang/crates.io-index".to_string()))
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

    pub async fn get_registry_url(&mut self) -> reqwest::Result<()> {
        let url = self.kind.crates_io_registry_url();

        let res: CratesIoCallRequest = reqwest::get(url)
            .await?
            .json()
            .await
            .expect("Could not deserialize crates-io kind");

        self.root_url = res.crate_map.homepage;

        Ok(())
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

#[derive(Debug, Clone)]
pub struct GithubApiUrls {
    pub root_url:             String,
    pub file_tree_url:        String,
    pub base_contents_url:    String,
    pub commit:               String,
    pub file_cache_path:      String,
    pub root_file_cache_path: String,
}

impl GithubApiUrls {
    pub fn check_file_cache(&self) -> bool {
        let redefined_file_cache = std::path::Path::new(&self.root_file_cache_path);
        if redefined_file_cache.exists() && redefined_file_cache.is_dir() {
            let file_path = std::path::Path::new(&self.file_cache_path);
            if file_path.exists() {
                return true
            }
            return false
        } else {
            std::fs::create_dir_all(&redefined_file_cache).expect("Failed to create the redefined file cache");
            return false
        }
    }

    pub async fn get_all_urls(&self, web_client: &reqwest::Client) -> reqwest::Result<Vec<String>> {
        let tree_text = web_client
            .get(&self.file_tree_url)
            .header("User-Agent", "request")
            .send()
            .await?
            .text()
            .await
            .expect("Could not deserialize all url path results to text");

        let tree: GithubApiFileTree = serde_json::from_str(&tree_text)
            .expect(&format!("Could not deserialize all text of all url path results to text to GithubApiFileTree: {}", tree_text));

        let all_paths = tree
            .tree
            .into_iter()
            .map(|path| path.path)
            .filter(|p| p.ends_with(".rs"))
            .map(|path| format!("{}{path}?ref={}", self.base_contents_url, self.commit))
            .collect();

        Ok(all_paths)
    }
}

impl From<RemoteType> for GithubApiUrls {
    fn from(value: RemoteType) -> Self {
        let split_commit = value
            .package
            .root_url
            .split("#")
            .into_iter()
            .collect::<Vec<_>>();
        let commit = split_commit
            .last()
            .expect(&format!("Could not find github commit hash for package {:?}", value))
            .to_string();

        let mut split_owner = split_commit
            .first()
            .expect(&format!("Could not parse owner/repo for package {:?}", value))
            .split("/")
            .into_iter()
            .collect::<Vec<_>>();

        let repo = split_owner
            .pop()
            .expect(&format!("Could not parse repo for package {:?}", value))
            .to_string();

        let owner = split_owner
            .pop()
            .expect(&format!("Could not parse owner for package {:?}", value))
            .to_string();

        let file_tree_url = format!("https://api.github.com/repos/{owner}/{repo}/git/trees/{commit}?recursive=1");
        let base_contents_url = format!("https://raw.github.com/{owner}/{repo}/master/");

        let root_path = workspace_dir();
        let path = root_path.to_str().unwrap();

        let root_file_cache_path = format!("{path}/target/redefined_file_cache");
        let file_cache_path = format!("{root_file_cache_path}/{}_{owner}_{repo}_{commit}", value.name);

        Self { root_url: value.package.root_url, file_tree_url, base_contents_url, commit, file_cache_path, root_file_cache_path }
    }
}
