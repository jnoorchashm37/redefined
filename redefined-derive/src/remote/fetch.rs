use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::{future::join_all, stream::FuturesUnordered, Future, Stream, StreamExt};

use super::{file_cache::FileCache, file_parser::ParsedRemoteType, write_to_file_cache};

pub struct GithubFetcher<'a> {
    pub futs: FuturesUnordered<Pin<Box<dyn Future<Output = Vec<ParsedRemoteType>> + 'a>>>,
}

impl<'a> GithubFetcher<'a> {
    pub fn new() -> Self {
        let futs = FuturesUnordered::new();

        Self { futs }
    }

    pub fn spawn_all(&self, urls: &'a [(String, String)], web_client: &'a reqwest::Client, type_searched: &'a str, file_cache: &'a FileCache) {
        let chunks = urls.chunks(20);

        chunks.into_iter().for_each(|urls| {
            self.futs
                .push(Box::pin(Self::spawn_tasks(urls, web_client, type_searched, file_cache)))
        });
    }

    async fn spawn_tasks(
        urls: &[(String, String)],
        web_client: &reqwest::Client,
        type_searched: &str,
        file_cache: &FileCache,
    ) -> Vec<ParsedRemoteType> {
        join_all(urls.iter().map(|(url, file_cache_ext)| async move {
            let page_contents = web_client
                .get(&*url)
                .header("User-Agent", "request")
                .send()
                .await
                .expect(&format!("Could not complete request to github api for content results for url {url}"))
                .text()
                .await
                .expect(&format!("Could not deserialize github api content results as text for url {url}"));

            let file_cache_full_path = format!("{}/{}", file_cache.file_cache_path, file_cache_ext);
            write_to_file_cache(&file_cache_full_path, &page_contents);

            ParsedRemoteType::parse_from_page(url.to_string(), page_contents, type_searched)
        }))
        .await
        .into_iter()
        .flatten()
        .collect()
    }
}

impl Stream for GithubFetcher<'_> {
    type Item = Vec<ParsedRemoteType>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        while let Poll::Ready(Some(res)) = self.futs.poll_next_unpin(cx) {
            return Poll::Ready(Some(res))
        }

        if self.futs.is_empty() {
            return Poll::Ready(None)
        }

        Poll::Pending
    }
}
