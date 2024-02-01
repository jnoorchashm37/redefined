use std::{
    io::Read,
    pin::Pin,
    task::{Context, Poll},
};

use futures::{future::join_all, stream::FuturesUnordered, Future, Stream, StreamExt};

use super::types::{GithubApiFileTree, RemoteTypeMeta, StructOrEnum};

pub struct GithubFetcher<'fut> {
    pub futs: FuturesUnordered<Pin<Box<dyn Future<Output = Vec<RemoteTypeText>> + 'fut>>>,
}

impl<'fut> GithubFetcher<'fut> {
    pub fn new() -> Self {
        let futs = FuturesUnordered::new();

        Self { futs }
    }

    pub fn spawn_all(&self, urls: &'fut [String], web_client: &'fut reqwest::Client, type_searched: &'fut RemoteTypeMeta) {
        let chunks = urls.chunks(20);

        chunks.into_iter().for_each(|urls| {
            self.futs
                .push(Box::pin(Self::spawn_tasks(urls, web_client, type_searched)))
        });
    }

    async fn spawn_tasks(urls: &[String], web_client: &reqwest::Client, type_searched: &RemoteTypeMeta) -> Vec<RemoteTypeText> {
        join_all(urls.iter().map(|url| async move {
            let page_contents = web_client
                .get(&*url)
                .header("User-Agent", "request")
                .send()
                .await
                .expect(&format!("Could not complete request to github api for content results for url {url}"))
                .text()
                .await
                .expect(&format!("Could not deserialize github api content results as text for url {url}"));

            RemoteTypeText::parse_page(url.to_string(), page_contents, type_searched)
        }))
        .await
        .into_iter()
        .flatten()
        .collect()
    }
}

impl<'fut> Stream for GithubFetcher<'fut> {
    type Item = Vec<RemoteTypeText>;

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

#[derive(Debug, Clone)]
pub struct RemoteTypeText {
    pub url:       String,
    pub type_text: String,
    pub kind:      StructOrEnum,
}

impl RemoteTypeText {
    pub fn parse_file_cache(path: &str) -> Self {
        let mut file_text = String::new();
        std::fs::File::open(path)
            .expect(&format!("Unable to read the file cache at path: {:?}", path))
            .read_to_string(&mut file_text)
            .expect(&format!("Unable to read the file cache to string at path: {:?}", path));

        let kind = first_line_to_kind(
            &file_text
                .lines()
                .next()
                .expect(&format!("File is empty in file cache: {}", path)),
        );

        Self { url: path.to_string(), type_text: file_text, kind }
    }

    pub fn parse_page(url: String, page_contents: String, type_searched: &RemoteTypeMeta) -> Option<Self> {
        let mut lines = page_contents.lines();

        if let Some(first_line) = lines.find(|line| line_conditions(&line, type_searched)) {
            let mut struct_lines = first_line.to_string();

            let start_char = first_line_start_char(&struct_lines);
            let kind = first_line_to_kind(&struct_lines);

            // if url.contains("bind.rs") {
            //     panic!("HAHAHA\n {:?}\n\n{}", url, struct_lines);
            // }

            let mut closing_delimeter = '}';
            if matches!(kind, StructOrEnum::Struct) {
                let struct_kind = struct_kind(&struct_lines);

                if let Some(delimeter) = struct_kind.closing_delimiter() {
                    closing_delimeter = delimeter;
                } else {
                    return Some(Self { url, type_text: struct_lines, kind })
                }
            }

            while let Some(line) = lines.next() {
                if let Some(char) = line.chars().nth(start_char) {
                    if char == closing_delimeter {
                        struct_lines.push('\n');
                        struct_lines.push_str(line);
                        break;
                    }
                }

                struct_lines.push('\n');
                struct_lines.push_str(line);
            }

            return Some(Self { url, type_text: struct_lines, kind })
        }

        None
    }
}

fn line_conditions(line: &str, type_searched: &RemoteTypeMeta) -> bool {
    let is_struct = line.starts_with("struct ") || line.starts_with("pub struct ");

    let is_enum = line.starts_with("enum ") || line.starts_with("pub enum ");

    // visibility options
    let visibility = is_struct || is_enum;

    // without generics
    let case0 =
        visibility && (line.starts_with(&format!("enum {} ", type_searched.name)) || line.contains(&format!("struct {} ", type_searched.name))); // && !type_searched.has_generics;

    // with generics
    let case1 =
        visibility && (line.starts_with(&format!("enum {}<", type_searched.name)) || line.contains(&format!("struct {}<", type_searched.name))); // && type_searched.has_generics;

    //if line.contains("CompactUint") && line.starts_with("pub struct") {
    //   panic!("HAHAHA\n {}\n\nis_struct: {is_struct}\nis_enum: {is_enum}\nvisibility: {visibility}\ncase0:{case0}\ncase1: {case1}", line);
    //}

    case0 || case1
}

fn first_line_start_char(line: &str) -> usize {
    line.chars()
        .enumerate()
        .find(|(_, c)| *c != ' ')
        .expect(&format!("Failed to parse the beginning of the first line of the type: {line}"))
        .0
}

fn first_line_to_kind(line: &str) -> StructOrEnum {
    if line.contains("struct ") || line.contains("pub struct ") {
        StructOrEnum::Struct
    } else if line.contains("enum ") || line.contains("pub enum ") {
        StructOrEnum::Enum
    } else {
        panic!("Expected 'Struct' or 'Enum'")
    }
}

fn struct_kind(line: &str) -> StructTypeHelper {
    if line.contains("{") {
        StructTypeHelper::Bracketed
    } else if line.contains("(") && !line.contains(")") && !line.contains(";") {
        StructTypeHelper::MultiLinesBraced
    } else if line.contains("(") && line.contains(")") && line.contains(";") {
        StructTypeHelper::SingleLineBraced
    } else {
        panic!("Unable to get struct kind for line: {}", line)
    }
}

enum StructTypeHelper {
    /// struct A {
    ///     ...
    /// }
    Bracketed,
    /// struct A (
    ///     ...
    /// );
    MultiLinesBraced,
    /// struct A(...);
    SingleLineBraced,
}

impl StructTypeHelper {
    fn closing_delimiter(&self) -> Option<char> {
        match self {
            StructTypeHelper::Bracketed => Some('}'),
            StructTypeHelper::MultiLinesBraced => Some(')'),
            StructTypeHelper::SingleLineBraced => None,
        }
    }
}
