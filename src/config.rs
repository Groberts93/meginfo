//! Set up configuration for CLI binary.

use std::path::PathBuf;

use anyhow::anyhow;

use crate::enums::DataTagKind;
use crate::tag::{self, TagDef};

use anyhow::Result;

#[derive(Debug)]
pub struct Config {
    pub files: Vec<PathBuf>,
    pub query_codes: Vec<DataTagKind>,
    pub show_tree: bool,
    pub describe_tags: Vec<TagDef>,
}

impl Config {
    pub fn new(
        files: Vec<PathBuf>,
        show_tree: bool,
        query_tags: Vec<String>,
        describe: bool,
    ) -> Result<Config> {
        let string_to_tag = tag::read_tag_dict();
        let query_codes: Result<Vec<&TagDef>> = query_tags
            .iter()
            .map(|x| {
                string_to_tag.get(x).ok_or(anyhow!(
                    "Unrecognized tag: {:?}. See fiff/tags.tsv for a list of valid names.",
                    x
                ))
            })
            .collect();

        let query_codes = query_codes?;

        let describe_tags: Vec<TagDef> = if describe {
            query_codes.clone().into_iter().cloned().collect()
        } else {
            vec![]
        };

        let query_codes: Result<Vec<DataTagKind>> = query_codes
            .into_iter()
            .map(|x| DataTagKind::from_code(x.code))
            .collect();

        let query_codes = query_codes?;

        Ok(Config {
            files,
            show_tree,
            query_codes,
            describe_tags,
        })
    }
}
