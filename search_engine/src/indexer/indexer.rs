use io::Result;
use std::{env, fs, io};
use std::borrow::Borrow;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Write};
use std::path::Path;

use amxml::sax::{SaxDecoder, XmlToken};
use stopwords::{Language, Spark, Stopwords};

use crate::indexer::category::Region;
use crate::indexer::index_file::IndexFile;
use crate::indexer::IndexerTags;
use crate::indexer::parser::Parser;

static PAGE_LIMIT: u32 = 2500;
static SECONDARY_LIMIT: u32 = 1000;
static TERTIARY_LIMIT: u32 = 100;

pub struct Indexer {
    index: BTreeMap<String, HashMap<String, IndexerTags>>,
    title_map: BTreeMap<u64, String>,
    page_counter: u64,
    secondary_index_count: u64,
    tertiary_index_count: u64,
    secondary_offset: u64,
    tertiary_offset: u64,
    secondary_index_to_store: u64,
    tertiary_index_to_store: u64,
    is_title: bool,
    is_page: bool,
    is_revision: bool,
    is_id: bool,
    is_text: bool,
    title_counter: u32,
    index_counter: u32,
    buffer_title: String,
    buffer_text: String,
    buffer_id: String,
    id_data: String,
    title_data: String,
    index_file: IndexFile,
    parser: Parser,
}

impl Indexer {
    pub fn new() -> Self {
        Indexer {
            index: BTreeMap::new(),
            title_map: BTreeMap::new(),
            page_counter: 0,
            secondary_index_count: 0,
            tertiary_index_count: 0,
            secondary_offset: 0,
            tertiary_offset: 0,
            secondary_index_to_store: 0,
            tertiary_index_to_store: 0,
            is_title: false,
            is_page: false,
            is_revision: false,
            is_id: false,
            is_text: false,
            title_counter: 0,
            index_counter: 0,
            buffer_title: "".to_string(),
            buffer_text: "".to_string(),
            buffer_id: "".to_string(),
            id_data: "".to_string(),
            title_data: "".to_string(),
            index_file: IndexFile::new(),
            parser: Parser::new(),
        }
    }

    pub fn build_index(&mut self, wiki_string: &str) {
        let mut dec = SaxDecoder::new(wiki_string).unwrap();
        loop {
            match dec.raw_token() {
                Ok(XmlToken::EOF) => {
                    self.create_index();
                    break;
                }
                Ok(XmlToken::StartElement { name, attr }) => {
                    match &name[..] {
                        "title" => {
                            self.is_title = true;
                            self.buffer_title = "".to_string();
                        }
                        "id" => {
                            if !self.is_page {
                                self.is_id = true;
                                self.is_page = true;
                                self.buffer_id = String::from("");
                            }
                        }
                        "text" => {
                            self.is_text = true;
                            self.buffer_text = "".to_string();
                        }
                        _ => {}
                    }
                }
                Ok(XmlToken::EndElement { name }) => {
                    match &name[..] {
                        "title" => {
                            self.is_title = false;
                            for (word, region) in self.parser.parse_title(&self.buffer_title) {
                                self.add_property(&*word, region)
                            }
                        }
                        "page" => {
                            self.is_page = false;
                            self.index_counter += 1;
                            self.title_counter += 1;
                            if self.page_counter == PAGE_LIMIT as u64 {
                                self.create_index();
                                self.page_counter = 0;
                            }
                        }
                        "id" => self.is_id = false,
                        "text" => {
                            self.is_text = false;
                            for (word, region) in self.parser.parse_text(&self.buffer_text) {
                                self.add_property(&word, region)
                            }
                        }
                        _ => {}
                    }
                }
                Ok(XmlToken::CharData { chardata }) => {
                    if chardata.trim().len() == 0 {
                        continue;
                    }
                    if self.is_id {
                        self.buffer_id = chardata.clone();
                        if self.is_page {
                            self.title_map.insert(self.buffer_id.parse::<u64>().unwrap(), self.buffer_title.clone());
                        }
                    } else if self.is_title {
                        self.buffer_title = format!("{}", chardata);
                    } else if self.is_text {
                        self.buffer_text = format!("{}", chardata);
                    }
                }
                _ => {}
            }
        }
    }

    pub fn add_property(&mut self, str: &str, region: Region) {
        match self._process(str) {
            Some(indexer_tags) => {
                match region {
                    Region::Title => indexer_tags.increment_title(),
                    Region::Body => indexer_tags.increment_body(),
                    Region::Category => indexer_tags.increment_category(),
                    Region::InfoBox => indexer_tags.increment_info_box(),
                }
            }
            _ => {}
        }
    }

    pub fn create_index(&mut self) {
        self.index_file.create(self.page_counter, &self.index, &self.title_map);
        self.page_counter += 1;
    }

    fn _process(&mut self, str: &str) -> Option<&mut IndexerTags> {
        let key = str.to_string();
        if !self.index.contains_key(&key) {
            self.index.insert(key.clone(), HashMap::new());
        }

        if !self.index.get(&key)?.contains_key(&self.buffer_id) {
            self.index.get_mut(&key)?.insert(self.buffer_id.to_string(), IndexerTags::new());
        }

        self.index.get_mut(&key)?.get_mut(&*self.buffer_id)
    }
}
