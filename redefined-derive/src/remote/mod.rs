mod fetch;
mod package;
mod types;

use std::io::Write;

use futures::StreamExt;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{spanned::Spanned, DeriveInput};

use self::{fetch::RemoteTypeText, package::GithubApiUrls, types::RemoteType};
use crate::remote::{fetch::GithubFetcher, types::RemoteTypeMeta};

pub fn expand_redefined_remote(input: TokenStream) -> syn::Result<TokenStream> {
    let mut parsed: RemoteType = syn::parse2(input)?;
    let remote_type_meta = RemoteTypeMeta::new(&parsed);

    let (remote_type_text, file_cache_path_to_write) = get_remote_type(&mut parsed, &remote_type_meta);

    let tokens = parse_remote_type_text(&remote_type_text, &remote_type_meta);

    if let Some(path) = file_cache_path_to_write {
        write_to_file_cache(&path, &remote_type_text);
    }

    tokens
}

fn parse_remote_type_text(remote_type_text: &str, meta: &RemoteTypeMeta) -> syn::Result<TokenStream> {
    let remote_type_text = remote_type_text.replace(&meta.name, &format!("{}Redefined", meta.name));

    let struct_def: DeriveInput = syn::parse_str(&remote_type_text)?;

    let remote_type = Ident::new(&meta.name, struct_def.span());
    let tokens = quote! {

        #[derive(Redefined)]
        #[redefined(#remote_type)]
        #[redefined_attr(transmute)]
        #struct_def
    };

    Ok(tokens)
}

fn get_remote_type(parsed: &mut RemoteType, remote_type_meta: &RemoteTypeMeta) -> (String, Option<String>) {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Could not build tokio rt");

    let web_client = reqwest::Client::new();
    if parsed.package.kind.is_crates_io() {
        parsed.package.root_url = rt
            .block_on(parsed.package.get_registry_url(&web_client))
            .expect("Could not get registry url");
    }

    let github_api_urls: GithubApiUrls = parsed.clone().into();

    let file_cache_path = github_api_urls.file_cache_path.clone();
    if github_api_urls.check_file_exists() {
        return (RemoteTypeText::parse_file_cache(&github_api_urls.file_cache_path).type_text, None)
    }
    if let Some((result, path)) = github_api_urls.fetch_from_file_cache(remote_type_meta) {
        (result.type_text, Some(format!("{path}/{}", remote_type_meta.name)))
    } else {
        let all_urls = rt
            .block_on(github_api_urls.get_all_urls(&web_client))
            .expect(&format!("Could not get url github urls for package: {:?}", parsed));

        let mut fetcher = GithubFetcher::new();

        fetcher.spawn_all(&all_urls, &web_client, &remote_type_meta, &file_cache_path);

        let fut = async {
            let mut results = Vec::new();
            while let Some(vals) = fetcher.next().await {
                results.extend(vals);
            }

            results
        };

        let results = rt.block_on(fut);

        if results.len() == 0 {
            panic!("No Results From Github For Package: {:?}", parsed);
        } else if results.len() > 1 {
            panic!("Too Many Results From Github For Package: {:?}\nResults: {:?}", parsed, results);
        }

        (results.first().unwrap().type_text.clone(), Some(github_api_urls.file_cache_path))
    }
}

pub fn write_to_file_cache(path: &str, text: &str) {
    let mut file = std::fs::File::create(path).expect(&format!("Failed to open file in file cache: {}", path));

    file.write_all(text.as_bytes())
        .expect(&format!("Failed to write file in file cache: {}", path));
}
