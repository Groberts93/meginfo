use std::collections::{HashMap, HashSet};

use csv::WriterBuilder;

use crate::{
    enums::DataTagKind,
    tag::{Data, Tag},
};

type QuerySet = HashSet<DataTagKind>;
type ResultSet = HashMap<DataTagKind, Vec<Data>>;

pub struct Search {
    state: SearchState,
}

impl Search {
    pub fn new(codes: QuerySet) -> Self {
        Search {
            state: SearchState::new(codes),
        }
    }

    pub fn execute(&mut self, tags: Vec<Tag>) -> ResultSet {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
enum SearchState {
    Pending(QuerySet),
    Complete(ResultSet),
}

impl SearchState {
    pub fn new(codes: QuerySet) -> Self {
        SearchState::Pending(codes)
    }
}

#[cfg(test)]
mod tests {
    use crate::tag::Data;

    use super::*;

    #[test]
    fn can_create_search() {
        let search = Search::new(default_set());

        assert_eq!(search.state, SearchState::Pending(default_set()));
    }

    #[test]
    fn can_execute_search() {
        let mut search = Search::new(default_set());

        search.execute(default_tags());

        assert_eq!(search.state, SearchState::Complete(default_results()))
    }

    fn default_set() -> HashSet<DataTagKind> {
        let tags = vec![
            DataTagKind::FileId,
            DataTagKind::MeasDate,
            DataTagKind::Sfreq,
            DataTagKind::BadChs,
        ];

        HashSet::from_iter(tags.into_iter())
    }

    fn default_tags() -> Vec<Tag> {
        vec![
            Tag::Data {
                kind: DataTagKind::BadChs,
                data: Data::String("sensorA".into()),
            },
            Tag::Data {
                kind: DataTagKind::BadChs,
                data: Data::String("sensorB".into()),
            },
            Tag::Data {
                kind: DataTagKind::BadChs,
                data: Data::String("sensorC".into()),
            },
            Tag::Data {
                kind: DataTagKind::SphereLayers,
                data: Data::Int32(vec![3, 4, 5]),
            },
            Tag::Data {
                kind: DataTagKind::Sfreq,
                data: Data::Float(vec![200.0]),
            },
            Tag::Data {
                kind: DataTagKind::FileId,
                data: Data::Slice("test".into()),
            },
        ]
    }

    fn default_results() -> HashMap<DataTagKind, Vec<Data>> {
        let mut map = HashMap::new();

        map.insert(DataTagKind::FileId, vec![Data::Slice("test".into())]);

        map.insert(DataTagKind::SphereLayers, vec![Data::Int32(vec![3, 4, 5])]);

        map.insert(DataTagKind::Sfreq, vec![Data::Float(vec![200.0])]);

        map.insert(
            DataTagKind::BadChs,
            vec![
                Data::String("sensorA".into()),
                Data::String("sensorB".into()),
                Data::String("sensorC".into()),
            ],
        );

        map
    }
}
