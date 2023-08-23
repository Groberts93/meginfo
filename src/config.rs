use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::anyhow;
use csv::ReaderBuilder;

use crate::enums::DataTagKind;
use crate::tag::TagDef;

use anyhow::Result;

#[derive(Debug)]
pub struct Config {
    pub files: Vec<PathBuf>,
    pub show_tree: bool,
    pub query_codes: Vec<DataTagKind>,
}

impl Config {
    pub fn new(files: Vec<PathBuf>, show_tree: bool, query_tags: Vec<String>) -> Result<Config> {
        let string_to_tag = read_tag_dict();
        let query_codes: Result<Vec<&TagDef>> = query_tags
            .iter()
            .map(|x| {
                string_to_tag.get(x).ok_or(anyhow!(
                    "Unrecognized tag: {:?}. See fiff/tags.tsv for a list of valid names.",
                    x
                ))
            })
            .collect();

        let query_codes: Vec<DataTagKind> = query_codes?
            .into_iter()
            .map(|x| DataTagKind::from_code(x.code))
            .collect();

        Ok(Config {
            files,
            show_tree,
            query_codes,
        })
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
