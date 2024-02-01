use proc_macro2::Ident;
use serde::Deserialize;
use syn::{parse::Parse, LitStr, Token};

use super::package::Package;

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

#[derive(Deserialize)]
pub struct CratesIoCallRequest {
    #[serde(rename = "crate")]
    pub crate_map: CratesIoCall,
}

#[derive(Deserialize)]
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

#[derive(Debug, Clone, Deserialize)]
pub struct GithubApiFileContents {
    path:         String,
    download_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RemoteTypeMeta {
    pub name:         String,
    pub has_generics: bool,
}

impl RemoteTypeMeta {
    /// need to add func for generic remote types
    pub fn new(remote_type: &RemoteType) -> Self {
        Self { name: remote_type.name.to_string(), has_generics: false }
    }
}

#[derive(Debug, Clone)]
pub struct RemoteType {
    pub name:    Ident,
    pub package: Package,
}

impl Parse for RemoteType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: Ident = input
            .parse()
            .map_err(|e| syn::Error::new(e.span(), "Failed to parse name of remote type"))?;

        input.parse::<Token![:]>()?;

        let package_name: LitStr = input
            .parse()
            .map_err(|e| syn::Error::new(e.span(), "Failed to parse url of the remote type's crate/package"))?;

        let package = Package::new(package_name.value())
            .map_err(|_| syn::Error::new(package_name.span(), "Failed to parse the cargo lock for this package"))?;

        Ok(Self { name, package })
    }
}

#[derive(Debug, Clone)]
pub enum StructOrEnum {
    Struct,
    Enum,
}
