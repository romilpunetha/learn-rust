use std::{env, fs};
use std::fs::File;

use amxml::sax::*;

use crate::indexer::Indexer;

mod indexer;

fn main() {
    println!("Hello, world!");
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let wiki_xml = fs::read_to_string(format!("{}/wiki-demo-3.xml", public_path)).unwrap();
    let mut indexer = Indexer::new();
    indexer.build_index(&*wiki_xml);
}

