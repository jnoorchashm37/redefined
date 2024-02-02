use std::{
    fs::DirEntry,
    io::{BufRead, Read},
};

use super::{
    file_parser::ParsedRemoteType,
    types::{cargo_home_dir, get_all_files},
};
use crate::remote::types::workspace_dir;

/// parsing of the package from crates-io OR github
#[derive(Debug, Clone)]
pub struct Package {
    pub package_name: String,
    pub version:      String,
    pub kind:         PackageKind,
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
                    let kind = if line.contains("https://github.com/rust-lang/crates.io-index") {
                        PackageKind::CratesIo(package.clone())
                    } else {
                        PackageKind::parse_github_repo(&line.replace("source = ", "").replace("git+", ""))
                    };

                    return Ok(Package {
                        package_name: package.clone(),
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

    /// attempts the fetch the type from the cached files of the repo
    pub fn fetch_from_file_cache(&self, type_searched: &str) -> ParsedRemoteType {
        let package_dir = self.kind.fetch_from_cargo(&self.version);
        //panic!("PATH: {:?}", package_dir.path().as_os_str());

        let mut paths = Vec::new();
        get_all_files(&package_dir, &mut paths);

        let results = paths
            .into_iter()
            .filter_map(|path| {
                let mut file = std::fs::File::open(&path).expect(&format!("Could not open file {:?} from cargo file cache", &path));
                let mut file_contents = String::new();
                file.read_to_string(&mut file_contents)
                    .expect(&format!("Could not read file {:?} to string", path));

                let p = path.as_path().to_str().unwrap().to_string();
                ParsedRemoteType::parse_from_page(p.clone(), file_contents, type_searched)
            })
            .collect::<Vec<_>>();

        if results.len() == 0 {
            panic!("No Results From File Cache For Package: {:?}", self);
        } else if results.len() > 1 {
            panic!("Too Many Results From File Cache For Package: {:?}\nResults: {:?}", self, results);
        } else {
            results.first().unwrap().clone()
        }
    }
}

#[derive(Debug, Clone)]
pub enum PackageKind {
    CratesIo(String),
    Github(String, String),
}

impl PackageKind {
    fn parse_github_repo(url: &str) -> Self {
        let split_commit = url.split("#").collect::<Vec<_>>();
        let commit = split_commit
            .last()
            .expect(&format!("Could not find github commit hash for package {:?}", url))
            .to_string();

        let mut split_owner = split_commit
            .first()
            .expect(&format!("Could not parse owner/repo for package {:?}", url))
            .split("/")
            .collect::<Vec<_>>();

        let repo_and_query = split_owner
            .pop()
            .expect(&format!("Could not parse repo/query for package {:?}", url))
            .split("?")
            .collect::<Vec<_>>();

        let repo = repo_and_query
            .first()
            .expect(&format!("Could not parse repo for package {:?}", url))
            .to_string()
            .replace(".git", "");

        Self::Github(repo, commit)
    }

    fn fetch_from_cargo(&self, version: &str) -> DirEntry {
        match self {
            PackageKind::CratesIo(_) => self.fetch_from_cargo_crates_io(version),
            PackageKind::Github(..) => self.fetch_from_cargo_git(),
        }
    }

    fn fetch_from_cargo_crates_io(&self, version: &str) -> DirEntry {
        let package_name = match self {
            PackageKind::CratesIo(p) => p,
            _ => unreachable!("cannot fetch from github for crates-io"),
        };

        let mut cargo_dir_path = cargo_home_dir();
        cargo_dir_path.push("registry/src");

        let all_dirs = std::fs::read_dir(&cargo_dir_path)
            .expect(&format!("Could not read cargo crates-io dir for {:?}", cargo_dir_path))
            .collect::<Result<Vec<_>, _>>()
            .expect(&format!("Coult not read subdirectories for: {:?}", cargo_dir_path));

        let dir_value = all_dirs
            .first()
            .expect(&format!("No subdirectories for: {:?}", cargo_dir_path));
        let crates_io_path = dir_value.path();

        let project_crate = std::fs::read_dir(&crates_io_path)
            .expect(&format!("Could not read cargo crates-io dir for {:?}", crates_io_path))
            .collect::<Result<Vec<_>, _>>()
            .expect(&format!("Coult not read subdirectories for: {:?}", crates_io_path))
            .into_iter()
            .find(|c| c.path().is_dir() && c.file_name().to_str().unwrap() == &format!("{package_name}-{version}"))
            .expect(&format!("Could not find crates-io package with name: {package_name}-{version}"));

        project_crate
    }

    fn fetch_from_cargo_git(&self) -> DirEntry {
        let (package_name, commit) = match self {
            PackageKind::Github(p, c) => (p, c),
            _ => unreachable!("cannot fetch for from github"),
        };

        let mut cargo_dir_path = cargo_home_dir();
        cargo_dir_path.push("git/checkouts");

        let dir_value = std::fs::read_dir(&cargo_dir_path)
            .expect(&format!("Could not read cargo git dir for {:?}", cargo_dir_path))
            .collect::<Result<Vec<_>, _>>()
            .expect(&format!("Coult not read subdirectories for: {:?}", cargo_dir_path))
            .into_iter()
            .find(|sub_dir| {
                let file_name = sub_dir.file_name();
                let p = file_name.to_str().unwrap();
                p.starts_with(&format!("{package_name}-")) && p.replace(&format!("{package_name}-"), "").len() == 16
            })
            .expect(&format!("Could not find git package directory with name: {package_name}"));

        let git_path = dir_value.path();

        // 16

        let project_crate = std::fs::read_dir(&git_path)
            .expect(&format!("Could not read cargo git dir for {:?}", git_path))
            .collect::<Result<Vec<_>, _>>()
            .expect(&format!("Coult not read subdirectories for: {:?}", git_path))
            .into_iter()
            .find(|c| c.path().is_dir() && c.file_name().to_str().unwrap() == &format!("{}", commit[0..7].to_string()))
            .expect(&format!("Could not find git package with name: {}", commit[0..7].to_string()));

        project_crate
    }
}
