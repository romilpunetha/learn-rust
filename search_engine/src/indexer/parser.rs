use std::collections::HashSet;

use porter_stemmer::stem;
use regex::Regex;
use stopwords::{Language, Spark, Stopwords};

use crate::indexer::category::Region;

pub struct Parser {
    stop_words: HashSet<String>,
    regex: Regex,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            stop_words: Spark::stopwords(Language::English).unwrap().iter().map(|i| i.to_string()).collect(),
            regex: Regex::new("[:/'\"~.\\-%()*|<>,=!]").unwrap(),
        }
    }

    pub fn parse_title(&self, title: &str) -> Vec<(String, Region)> {
        self.regex.replace_all(title, " ").chars()
            .filter(|c| c.is_ascii() && (c.is_whitespace() || c.is_alphanumeric()))
            .collect::<String>().trim()
            .to_lowercase()
            .split_whitespace()
            .filter(|x| !self.stop_words.contains(*x) && x.len() > 0)
            .map(|x| (stem(x).to_string(), Region::Title))
            .collect()
    }

    pub fn parse_text(&self, text: &str) -> Vec<(String, Region)> {
        let text = self.regex.replace_all(text, " ")
            .to_string()
            .to_lowercase();

        let (mut info_box, text) = self._parse_infobox(text);
        let (mut category, text) = self._parse_category(text);
        let mut body: Vec<(String, Region)> = self._parse_body(text);

        info_box.append(&mut category);
        info_box.append(&mut body);

        info_box
    }

    fn _parse_infobox(&self, mut text: String) -> (Vec<(String, Region)>, String) {
        let mut response: Vec<(String, Region)> = Vec::new();

        let info_box_count = text.matches("{{infobox").count();

        for k in 0..info_box_count {
            let i = text.find("{{infobox").unwrap();

            let mut j = i + 1;
            loop {
                let count_open_bracket = text[i..j].matches("{{").count();
                let count_close_bracket = text[i..j].matches("}}").count();

                if count_open_bracket > 0 && count_open_bracket == count_close_bracket {
                    break;
                }
                j += 1;
            }

            response.append(&mut text[i..j].chars()
                .filter(|c| c.is_ascii() && (c.is_whitespace() || c.is_alphanumeric()))
                .collect::<String>()
                .split_whitespace()
                .filter(|x| !self.stop_words.contains(*x) && x.len() > 0)
                .map(|x| (stem(x).trim().to_string(), Region::InfoBox))
                .collect::<Vec<(String, Region)>>());

            text.replace_range((i..j), "");
        }
        (response, text)
    }

    fn _parse_category(&self, mut text: String) -> (Vec<(String, Region)>, String) {
        let mut response: Vec<(String, Region)> = Vec::new();

        let info_box_count = text.matches("[[category").count();

        for k in 0..info_box_count {
            let i = text.find("[[category").unwrap();
            let mut j = i;

            loop {
                let count_open_bracket = text[i..j].matches("[[").count();
                let count_close_bracket = text[i..j].matches("]]").count();

                if count_open_bracket > 0 && count_open_bracket == count_close_bracket {
                    break;
                }
                j += 1;
            }

            response.append(&mut text[i..j].chars()
                .filter(|c| c.is_ascii() && (c.is_whitespace() || c.is_alphanumeric()))
                .collect::<String>()
                .split_whitespace()
                .filter(|x| !self.stop_words.contains(*x) && x.len() > 0)
                .map(|x| (stem(x).trim().to_string(), Region::Category))
                .collect::<Vec<(String, Region)>>());

            text.replace_range((i..j), "");
        }
        (response, text)
    }

    fn _parse_body(&self, mut text: String) -> Vec<(String, Region)> {
        text.chars()
            .filter(|c| c.is_ascii() && (c.is_whitespace() || c.is_alphanumeric()))
            .collect::<String>()
            .split_whitespace()
            .filter(|x| !self.stop_words.contains(*x) && x.len() > 0)
            .map(|x| (stem(x).trim().to_string(), Region::Body))
            .collect()
    }
}