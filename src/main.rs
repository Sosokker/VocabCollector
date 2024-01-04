use std::fs::File;
use std::io::{self, Read};
use zip::read::ZipArchive;

pub mod utils {
    pub mod extractor;
    pub mod translator;
    pub mod xmlreader;
}

use utils::extractor::extract_text_from_html;
use utils::translator::{self, Translator};

fn read_epub(epub_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(epub_path)?;
    let mut archive = ZipArchive::new(file)?;

    let mut all_html_content = Vec::new();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;

        if file.name().to_lowercase().ends_with(".html") {
            let mut content = String::new();
            file.read_to_string(&mut content)?;

            all_html_content.push(content);
        }
    }

    Ok(all_html_content)
}

fn main() {
    let epub_path = "./data/example/data.epub";
    let translator = Translator::new(translator::Thai);

    match read_epub(epub_path) {
        Ok(all_html_content) => {
            for html_content in all_html_content {
                let unique_words = extract_text_from_html(&html_content);
                let translation_map = translator.translate(&unique_words);
                for (headword, translation) in &translation_map {
                    println!("Headword: {}, Translation: {}", headword, translation);
                }
            }
        }
        Err(e) => eprintln!("Error reading EPUB: {}", e),
    }
}
