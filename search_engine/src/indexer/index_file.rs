use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::Write;

use crate::Indexer;
use crate::indexer::IndexerTags;

pub struct IndexFile {
    create_page_count: u32,
}

impl IndexFile {
    pub fn new() -> Self {
        Self {
            create_page_count: 1
        }
    }

    pub fn create(&self, counter: u64, word_map: &BTreeMap<String, HashMap<String, IndexerTags>>, title_map: &BTreeMap<u64, String>) {
        self._create_title_file(&counter, title_map);
        self._create_index_file(&counter, word_map);
    }

    fn _create_index_file(&self, counter: &u64, word_map: &BTreeMap<String, HashMap<String, IndexerTags>>) {
        let mut index = File::options().append(true).create(true).open(format!("{}/public/{}.txt", env!("CARGO_MANIFEST_DIR"), counter)).unwrap();
        for entry in word_map {
            let word = entry.0;

            let word_index = entry.1;
            index.write_all(format!("{}-", word).as_ref()).unwrap();

            for loc_entry in word_index {
                let document_id = loc_entry.0;
                let indexer_tags = loc_entry.1;
                let idf = indexer_tags.get_title() + indexer_tags.get_body() + indexer_tags.get_category() + indexer_tags.get_info_box();
                let mut to_append: String = format!("{}:f{}", document_id, idf);
                if indexer_tags.get_title() != 0 {
                    to_append = format!("{}t{}", to_append, indexer_tags.get_title());
                }
                if indexer_tags.get_body() != 0 {
                    to_append = format!("{}b{}", to_append, indexer_tags.get_body());
                }
                if indexer_tags.get_category() != 0 {
                    to_append = format!("{}c{}", to_append, indexer_tags.get_category());
                }
                if indexer_tags.get_info_box() != 0 {
                    to_append = format!("{}i{}", to_append, indexer_tags.get_info_box());
                }
                to_append = format!("{};", to_append);
                index.write_all(to_append.as_ref()).unwrap();
            }
            index.write_all(b"\r\n").unwrap();
        }
    }

    fn _create_title_file(&self, counter: &u64, title_map: &BTreeMap<u64, String>) {
        let mut index = File::options().append(true).create(true).open(format!("{}/public/title{}.txt", env!("CARGO_MANIFEST_DIR"), counter)).unwrap();
        for entry in title_map {
            index.write_all(format!("{}-{}\n", entry.0, entry.1).as_ref()).unwrap();
        }
    }
}