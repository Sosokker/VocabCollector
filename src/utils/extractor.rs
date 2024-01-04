use select::document::Document;
use select::predicate::Name;
use std::collections::HashSet;

pub fn extract_text_from_html(html_content: &str) -> Vec<String> {
    let document = Document::from_read(html_content.as_bytes()).unwrap();

    let mut unique_words = HashSet::new();

    for node in document.find(Name("text")) {
        let text = node.text();
        let lowercase_text = text.to_lowercase();
        unique_words.insert(lowercase_text);
    }

    let result: Vec<String> = unique_words.into_iter().collect();
    result
}
