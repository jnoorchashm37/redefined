use std::io::Read;

use super::types::StructOrEnum;

/// represents a parsed remote type
#[derive(Debug, Clone)]
pub struct ParsedRemoteType {
    /// the url or path
    pub url_or_path: String,
    /// the text of the struct/enum
    pub type_text:   String,
    /// where the parsed type is a struct or an enum
    pub kind:        StructOrEnum,
}

impl ParsedRemoteType {
    /// parse the text of a webpage
    pub fn parse_from_file_cache(path: &str) -> Self {
        let mut file_text = String::new();
        std::fs::File::open(path)
            .expect(&format!("Unable to read the file cache at path: {:?}", path))
            .read_to_string(&mut file_text)
            .expect(&format!("Unable to read the file cache to string at path: {:?}", path));

        let kind = StructOrEnum::new_from_line(
            &file_text
                .lines()
                .next()
                .expect(&format!("File is empty in file cache: {}", path)),
        );

        Self { url_or_path: path.to_string(), type_text: file_text, kind }
    }

    /// parse the text of a webpage
    pub fn parse_from_page(url: String, page_contents: String, type_searched: &str) -> Option<Self> {
        let mut lines = page_contents.lines();

        if let Some(first_line) = lines.find(|line| line_conditions(&line, type_searched)) {
            let mut struct_lines = first_line.to_string();

            let start_char = first_line_start_char(&struct_lines);
            let kind = StructOrEnum::new_from_line(&struct_lines);

            let mut closing_delimeter = '}';
            if matches!(kind, StructOrEnum::Struct) {
                let struct_kind = StructTypeHelper::new_from_line(&struct_lines);

                if let Some(delimeter) = struct_kind.closing_delimiter() {
                    closing_delimeter = delimeter;
                } else {
                    return Some(Self { url_or_path: url, type_text: struct_lines, kind })
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

            return Some(Self { url_or_path: url, type_text: struct_lines, kind })
        }

        None
    }
}

/// conditions whether a line is valid as the start of the target struct/enum
fn line_conditions(line: &str, type_searched: &str) -> bool {
    let is_struct = line.trim_start().starts_with("struct ") || line.trim_start().starts_with("pub struct ");

    let is_enum = line.trim_start().starts_with("enum ") || line.trim_start().starts_with("pub enum ");

    // visibility options
    let visibility = is_struct || is_enum;

    // without generics
    let case0 = visibility && (line.contains(&format!("enum {} ", type_searched)) || line.contains(&format!("struct {} ", type_searched)));

    // with generics
    let case1 = visibility && (line.contains(&format!("enum {}<", type_searched)) || line.contains(&format!("struct {}<", type_searched)));

    case0 || case1
}

/// finds the index of the first character of the struct/enum
fn first_line_start_char(line: &str) -> usize {
    line.chars()
        .enumerate()
        .find(|(_, c)| *c != ' ')
        .expect(&format!("Failed to parse the beginning of the first line of the type: {line}"))
        .0
}

/// helper to determine how to format the struct
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
    fn new_from_line(line: &str) -> Self {
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

    fn closing_delimiter(&self) -> Option<char> {
        match self {
            StructTypeHelper::Bracketed => Some('}'),
            StructTypeHelper::MultiLinesBraced => Some(')'),
            StructTypeHelper::SingleLineBraced => None,
        }
    }
}
