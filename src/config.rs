use std::collections::HashMap;
use std::path::PathBuf;

use csv::ReaderBuilder;

use crate::tag::TagDef;

#[derive(Debug)]
pub struct Config {
    pub files: Vec<PathBuf>,
    pub show_tree: bool,
    pub query_codes: Vec<i32>,
    pub debug: bool
}

impl Config {
    pub fn new(files: Vec<PathBuf>, show_tree: bool, query_tags: Vec<String>, debug: bool) -> Config {
        let string_to_tag = read_tag_dict();
        let query_codes = query_tags
            .iter()
            .map(|x| string_to_tag.get(x).unwrap().code)
            .collect();

        Config {
            files,
            show_tree,
            query_codes,
            debug
        }
    }
}

fn read_tag_dict() -> HashMap<String, TagDef> {
    let mut reader = ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path("fiff/tags.tsv")
        .expect("file should be found in fiff/tags.tsv");

    let mut string_to_tag: HashMap<String, TagDef> = HashMap::new();

    for result in reader.deserialize() {
        let record: TagDef = result.expect("static tsv should have been readable");
        string_to_tag.insert(record.name.clone(), record);
    }

    string_to_tag
}
