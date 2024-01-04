use std::collections::HashMap;

use super::xmlreader::read_xml;

pub trait Language {
    fn translate(&self, word: &Vec<String>) -> HashMap<String, String>;
}

pub struct Thai;

impl Language for Thai {
    fn translate(&self, words: &Vec<String>) -> HashMap<String, String> {
        let words_map = read_xml("./data/words/th-en.xml");
        let mut translation_result: HashMap<String, String> = HashMap::new();

        for word in words {
            if let Some(translation) = words_map.get(word) {
                translation_result.insert(word.clone(), translation.clone());
            }
        }

        translation_result
    }
}

pub struct Translator<T: Language> {
    language: T,
}

impl<T: Language> Translator<T> {
    pub fn new(language: T) -> Self {
        Self { language }
    }

    pub fn translate(&self, words: &Vec<String>) -> HashMap<String, String> {
        self.language.translate(words)
    }
}
