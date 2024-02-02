use std::io::{Read, Write};

use super::file_parser::ParsedRemoteType;

/// the file cache holds all cached results and files from different repos
/// formatted as: target/redefined_file_cache/{owner}_{repo}_{commit}
/// cached results are in the subpath: /cached/
/// cached files are in the subpath: /files/{relative_path_of_remote_file}
#[derive(Debug, Clone)]
pub struct FileCache {
    /// the path of the file with the cached result
    pub cached_file:          String,
    /// the path of the cached repo files
    pub file_cache_path:      String,
    /// the root path of the file cache
    pub root_file_cache_path: String,
}

impl FileCache {
    /// whether or not the cached result exists
    pub fn check_file_exists(&self) -> bool {
        let redefined_file_cache = std::path::Path::new(&self.root_file_cache_path);
        if redefined_file_cache.exists() && redefined_file_cache.is_dir() {
            let repo_cached_results_path_str = self.file_cache_path.replace("/files", "/cached");
            let repo_cached_results_path = std::path::Path::new(&repo_cached_results_path_str);
            if repo_cached_results_path.exists() && repo_cached_results_path.is_dir() {
                let file_path = std::path::Path::new(&self.cached_file);
                if file_path.exists() {
                    return true
                }
            } else {
                std::fs::create_dir_all(&repo_cached_results_path)
                    .expect(&format!("Failed to create the redefined file cache for repo: {}", repo_cached_results_path_str));
            }

            return false
        } else {
            std::fs::create_dir_all(&redefined_file_cache)
                .expect(&format!("Failed to create the redefined file cache: {}", self.root_file_cache_path));
            return false
        }
    }

    /// attemos the fetch the type from the cached files of the repo
    pub fn fetch_from_file_cache(&self, type_searched: &str, sub_path: &Option<String>) -> Option<ParsedRemoteType> {
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

        let sp = sub_path.clone().map(|s| s.replace("-", "_"));
        for entry in dir_values {
            let entry = entry.expect(&format!("Could not get file cache dir entry for {}", self.file_cache_path));
            let path = entry.path();

            if path.is_file() {
                if let Some(ref p) = sp {
                    if !path.as_path().to_str().unwrap().contains(p) {
                        continue;
                    }
                }
                let mut file =
                    std::fs::File::open(&path).expect(&format!("Could not open file {:?} from file cache for {}", &path, self.file_cache_path));
                let mut file_contents = String::new();
                file.read_to_string(&mut file_contents)
                    .expect(&format!("Could not read file {:?} to string for {}", path, self.file_cache_path));

                let p = path.as_path().to_str().unwrap().to_string();
                if let Some(r) = ParsedRemoteType::parse_from_page(p.clone(), file_contents, type_searched) {
                    results.push(r)
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

pub fn write_to_file_cache(path: &str, text: &str) {
    let mut file = std::fs::File::create(path).expect(&format!("Failed to open file in file cache: {}", path));

    file.write_all(text.as_bytes())
        .expect(&format!("Failed to write file in file cache: {}", path));
}
