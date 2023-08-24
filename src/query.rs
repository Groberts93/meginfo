use std::collections::{HashMap, HashSet};

use csv::WriterBuilder;

use crate::{enums::DataTagKind, tag::Tag};

pub struct Search {
    state: SearchState,
}

impl Search {
    pub fn new(codes: HashSet<DataTagKind>) -> Self {
        Search {
            state: SearchState::new(codes),
        }
    }

    pub fn execute(&mut self) -> HashSet<Tag> {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
enum SearchState {
    Pending(QuerySet),
    Complete(ResultSet),
}

impl SearchState {
    pub fn new(codes: HashSet<DataTagKind>) -> Self {
        SearchState::Pending(QuerySet { tags: codes })
    }
}

#[derive(Debug, PartialEq)]
struct QuerySet {
    tags: HashSet<DataTagKind>,
}

#[derive(Debug, PartialEq)]
struct ResultSet {
    tags: HashMap<DataTagKind, Vec<Tag>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_search() {
        let search = Search::new(default_set());

        assert_eq!(
            search.state,
            SearchState::Pending(QuerySet {
                tags: default_set()
            })
        );
    }

    #[test]
    fn can_execute_search() {
        let search = Search::new(default_set());


    }

    fn default_set() -> HashSet<DataTagKind> {
        let tags = vec![
            DataTagKind::FileId,
            DataTagKind::Sfreq,
            DataTagKind::MeasDate,
            DataTagKind::Sfreq,
        ];

        HashSet::from_iter(tags.into_iter())
    }

    fn default_tags() -> Vec<Tag> {

        // let tag = Tag::Data { kind: DataTagKind::BadChs, data:  }

        vec![]
    }
}
