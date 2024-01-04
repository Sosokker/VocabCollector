use serde::Deserialize;
use serde_xml_rs::from_reader;
use std::collections::HashMap;
use std::fs::File;

#[derive(Debug, Deserialize)]
pub struct Entry {
    pub translation: Translation,
    pub headword: String,
}

#[derive(Debug, Deserialize)]
pub struct Translation {
    #[serde(rename = "lang")]
    pub lang: String,
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct Lexitron {
    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct Lexicon {
    pub entry: Vec<Entry>,
}

pub fn read_xml(xml_path: &str) -> HashMap<String, String> {
    let xml_content = match File::open(xml_path) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error opening file: {}", xml_path);
            return HashMap::new();
        }
    };

    let lexicon: Lexicon = match from_reader(xml_content) {
        Ok(lex) => lex,
        Err(err) => {
            eprintln!("Error parsing XML: {:?}", err);
            return HashMap::new();
        }
    };

    let mut result_map: HashMap<String, String> = HashMap::new();

    for entry in lexicon.entry {
        result_map.insert(entry.headword, entry.translation.value);
    }

    result_map
}
