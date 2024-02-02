mod fetch;
mod file_cache;
mod file_parser;
mod package;
mod types;

use futures::StreamExt;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{bracketed, parenthesized, parse::Parse, spanned::Spanned, DeriveInput, LitStr, Token};

use self::{
    file_cache::{write_to_file_cache, FileCache},
    file_parser::ParsedRemoteType,
    package::{GithubApiUrls, Package},
    types::workspace_dir,
};
use crate::remote::fetch::GithubFetcher;

pub fn expand_redefined_remote(input: TokenStream) -> syn::Result<TokenStream> {
    let parsed: RemoteType = syn::parse2(input)?;

    parsed.execute()
}

/// parses the remote type into tokens
fn parse_remote_type_text(remote_type_name: &str, remote_type_text: &str, derives: Vec<Ident>) -> syn::Result<TokenStream> {
    let remote_type_text = remote_type_text.replace(remote_type_name, &format!("{}Redefined", remote_type_name));

    let struct_def: DeriveInput = syn::parse_str(&remote_type_text)?;

    let remote_type = Ident::new(remote_type_name, struct_def.span());
    let tokens = quote! {

        #[derive(#(#derives),*)]
        #[redefined(#remote_type)]
        #[redefined_attr(transmute)]
        #struct_def
    };

    Ok(tokens)
}

#[derive(Debug, Clone)]
pub struct RemoteType {
    pub name:    Ident,
    pub package: Package,
    pub derives: Vec<Ident>,
}

impl RemoteType {
    /// runs the remote type execution
    /// added for future use in fields of structs
    pub fn execute(mut self) -> syn::Result<TokenStream> {
        let derives = self.derives.clone();

        let (remote_type_text, file_cache_path_to_write) = self.get_remote_type();

        let tokens = parse_remote_type_text(&self.name.to_string(), &remote_type_text, derives);

        if let Some(path) = file_cache_path_to_write {
            write_to_file_cache(&path, &remote_type_text);
        }

        tokens
    }

    /// retrieves the remote type
    fn get_remote_type(&mut self) -> (String, Option<String>) {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Could not build tokio rt");

        let web_client = reqwest::Client::new();
        if self.package.kind.is_crates_io() {
            self.package.root_url = rt
                .block_on(self.package.get_registry_url(&web_client))
                .expect("Could not get registry url");
        }

        let (github_api_urls, file_cache): (GithubApiUrls, FileCache) = self.clone().into();

        let target_name = self.name.to_string();
        if file_cache.check_file_exists() {
            return (ParsedRemoteType::parse_from_file_cache(&file_cache.cached_file).type_text, None)
        }
        if let Some(result) = file_cache.fetch_from_file_cache(&self.name.to_string()) {
            (result.type_text, Some(file_cache.cached_file))
        } else {
            let all_urls = rt
                .block_on(github_api_urls.get_all_urls(&web_client))
                .expect(&format!("Could not get url github urls for package: {:?}", self));

            let mut fetcher = GithubFetcher::new();

            fetcher.spawn_all(&all_urls, &web_client, &target_name, &file_cache);

            let fut = async {
                let mut results = Vec::new();
                while let Some(vals) = fetcher.next().await {
                    results.extend(vals);
                }

                results
            };

            let results = rt.block_on(fut);

            if results.len() == 0 {
                panic!("No Results From Github For Package: {:?}", self);
            } else if results.len() > 1 {
                panic!("Too Many Results From Github For Package: {:?}\nResults: {:?}", self, results);
            }

            (results.first().unwrap().type_text.clone(), Some(file_cache.cached_file.to_string()))
        }
    }
}

impl Parse for RemoteType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut derives = vec![Ident::new("Redefined", input.span())];
        if input.peek(Token![#]) {
            input.parse::<Token![#]>()?; // #

            let bracketed_derive;
            bracketed!(bracketed_derive in input);
            bracketed_derive.parse::<Ident>()?; // derive

            let paran_derive;
            parenthesized!(paran_derive in bracketed_derive);

            derives.extend(
                paran_derive
                    .parse_terminated(Ident::parse, Token![,])?
                    .into_iter()
                    .collect::<Vec<_>>(),
            );
        }

        let name: Ident = input
            .parse()
            .map_err(|e| syn::Error::new(e.span(), "Failed to parse name of remote type"))?;

        input.parse::<Token![:]>()?;

        let package_name: LitStr = input
            .parse()
            .map_err(|e| syn::Error::new(e.span(), "Failed to parse url of the remote type's crate/package"))?;

        let package = Package::new(package_name.value())
            .map_err(|_| syn::Error::new(package_name.span(), "Failed to parse the cargo lock for this package"))?;

        Ok(Self { name, package, derives })
    }
}

impl Into<(GithubApiUrls, FileCache)> for RemoteType {
    fn into(self) -> (GithubApiUrls, FileCache) {
        let (commit, mut split_owner, is_crates_io) = if self.package.kind.is_crates_io() {
            let split_owner = self.package.root_url.split("/").collect::<Vec<_>>();

            ("main".to_string(), split_owner, true)
        } else {
            let split_commit = self.package.root_url.split("#").collect::<Vec<_>>();
            let commit = split_commit
                .last()
                .expect(&format!("Could not find github commit hash for package {:?}", self))
                .to_string();

            let split_owner = split_commit
                .first()
                .expect(&format!("Could not parse owner/repo for package {:?}", self))
                .split("/")
                .collect::<Vec<_>>();

            (commit, split_owner, false)
        };

        let repo_and_query = split_owner
            .pop()
            .expect(&format!("Could not parse repo/query for package {:?}", self))
            .split("?")
            .collect::<Vec<_>>();

        let repo = repo_and_query
            .first()
            .expect(&format!("Could not parse repo for package {:?}", self))
            .to_string()
            .replace(".git", "");

        //panic!("Could not parse owner for package {:?}", value.package.root_url);
        //panic!("Could not parse owner for package {:?}", split_owner);

        let owner = split_owner
            .pop()
            .expect(&format!("Could not parse owner for package {:?}", self))
            .to_string();

        let file_tree_url = format!("https://api.github.com/repos/{owner}/{repo}/git/trees/{commit}?recursive=1");
        let base_contents_url = format!("https://raw.github.com/{owner}/{repo}/{commit}/");

        let root_path = workspace_dir();
        let path = root_path.to_str().unwrap();

        let root_file_cache_path = format!("{path}/target/redefined_file_cache");
        let file_cache_path = format!("{root_file_cache_path}/{owner}_{repo}_{commit}/files");
        let cached_file = format!("{root_file_cache_path}/{owner}_{repo}_{commit}/cached/{}", self.name);

        let github_api_urls = GithubApiUrls { root_url: self.package.root_url, file_tree_url, base_contents_url, commit, is_crates_io };

        let file_cache = FileCache { cached_file, file_cache_path, root_file_cache_path };

        panic!("GITHUB: {:?}\n\nFILE CACHE: {:?}", github_api_urls, file_cache);

        (github_api_urls, file_cache)
    }
}
