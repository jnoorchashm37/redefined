use std::io::{BufRead, Read};

use super::{
    fetch::RemoteTypeText,
    types::{CratesIoCallRequest, RemoteType, RemoteTypeMeta},
};
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

        Ok(crates_io.crate_map.homepage)
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
    pub cached_file:          String,
    pub file_cache_path:      String,
    pub root_file_cache_path: String,
}

impl GithubApiUrls {
    /// whether or not the cached result exists
    pub fn check_file_exists(&self) -> bool {
        let redefined_file_cache = std::path::Path::new(&self.root_file_cache_path);
        if redefined_file_cache.exists() && redefined_file_cache.is_dir() {
            let file_path = std::path::Path::new(&self.cached_file);
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

    pub fn fetch_from_file_cache(&self, type_searched: &RemoteTypeMeta) -> Option<(RemoteTypeText, String)> {
        let redefined_file_cache = std::path::Path::new(&self.file_cache_path);
        if !redefined_file_cache.exists() || !redefined_file_cache.is_dir() {
            std::fs::create_dir_all(&redefined_file_cache).expect(&format!("Could not create file cache dir for {}", self.file_cache_path));
            return None
        }

        let mut results = Vec::new();
        let dir_values = std::fs::read_dir(redefined_file_cache)
            .expect(&format!("Could not read file cache dir for {}", self.file_cache_path))
            .collect::<Vec<_>>();
        if dir_values.is_empty() {
            return None
        }
        for entry in dir_values {
            let entry = entry.expect(&format!("Could not get file cache dir entry for {}", self.file_cache_path));
            let path = entry.path();

            // Check if the entry is a file
            if path.is_file() {
                // Open the file and read its contents
                let mut file =
                    std::fs::File::open(&path).expect(&format!("Could not open file {:?} from file cache for {}", &path, self.file_cache_path));
                let mut file_contents = String::new();
                file.read_to_string(&mut file_contents)
                    .expect(&format!("Could not read file {:?} to string for {}", path, self.file_cache_path));

                // Append the file contents to the main string
                let p = path.as_path().to_str().unwrap().to_string();
                if let Some(r) = RemoteTypeText::parse_page(p.clone(), file_contents, type_searched) {
                    results.push((r, p))
                }
            }
        }

        if results.len() == 0 {
            panic!("No Results From File Cache For Package: {:?}", self);
        } else if results.len() > 1 {
            panic!("Too Many Results From File Cache For Package: {:?}\nResults: {:?}", self, results);
        } else {
            Some(results.first().unwrap().clone())
        }
    }
}

impl From<RemoteType> for GithubApiUrls {
    fn from(value: RemoteType) -> Self {
        let (commit, mut split_owner) = if value.package.kind.is_crates_io() {
            let split_owner = value
                .package
                .root_url
                .split("/")
                .into_iter()
                .collect::<Vec<_>>();

            ("main".to_string(), split_owner)
        } else {
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

            let split_owner = split_commit
                .first()
                .expect(&format!("Could not parse owner/repo for package {:?}", value))
                .split("/")
                .into_iter()
                .collect::<Vec<_>>();

            (commit, split_owner)
        };

        let repo = split_owner
            .pop()
            .expect(&format!("Could not parse repo for package {:?}", value))
            .to_string();

        //panic!("Could not parse owner for package {:?}", value.package.root_url);
        //panic!("Could not parse owner for package {:?}", split_owner);

        let owner = split_owner
            .pop()
            .expect(&format!("Could not parse owner for package {:?}", value))
            .to_string();

        let file_tree_url = format!("https://api.github.com/repos/{owner}/{repo}/git/trees/{commit}?recursive=1");
        let base_contents_url = format!("https://raw.github.com/{owner}/{repo}/master/");

        let root_path = workspace_dir();
        let path = root_path.to_str().unwrap();

        let root_file_cache_path = format!("{path}/target/redefined_file_cache");
        let file_cache_path = format!("{root_file_cache_path}/{owner}_{repo}_{commit}/files");
        let cached_file = format!("{root_file_cache_path}/{owner}_{repo}_{commit}/cached/{}", value.name);

        Self { root_url: value.package.root_url, file_tree_url, base_contents_url, commit, cached_file, file_cache_path, root_file_cache_path }
    }
}
