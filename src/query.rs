use std::collections::{HashMap, HashSet};

use csv::WriterBuilder;

use crate::{
    enums::DataTagKind,
    tag::{Data, Tag},
};

type QuerySet = HashSet<DataTagKind>;
type ResultSet = HashMap<DataTagKind, Vec<Data>>;

pub struct Search {
    query: QuerySet,
    state: SearchState,
}

impl Search {
    pub fn new(codes: QuerySet) -> Self {
        Search {
            query: codes,
            state: SearchState::Pending,
        }
    }

    pub fn execute(&mut self, tags: Vec<Tag>) -> ResultSet {
        let mut results = ResultSet::new();

        for tag in tags {
            if let Tag::Data { kind, data } = tag {
                if self.query.contains(&kind) {
                    results
                        .entry(kind)
                        .and_modify(|x| x.push(data.clone()))
                        .or_insert(vec![data]);
                }
            }
        }

        self.state = SearchState::Complete(results.clone());

        results
    }
}

#[derive(Debug, PartialEq)]
enum SearchState {
    Pending,
    Complete(ResultSet),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tag::Data;

    #[test]
    fn can_create_search() {
        let search = Search::new(default_query());

        assert_eq!(search.state, SearchState::Pending);
        assert_eq!(search.query, default_query());
    }

    #[test]
    fn can_execute_search() {
        // this requires default_tags, default_results, and default_query to be correct
        let mut search = Search::new(default_query());
        let results = search.execute(default_tags());

        assert_eq!(search.state, SearchState::Complete(default_results()));
        assert_eq!(results, default_results());
    }

    fn default_query() -> QuerySet {
        let tags = vec![
            DataTagKind::FileId,
            DataTagKind::MeasDate,
            DataTagKind::Sfreq,
            DataTagKind::BadChs,
            DataTagKind::SphereLayers,
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
            Tag::Data {
                kind: DataTagKind::DacqPars,
                data: Data::Float(vec![1.5, 1.2, -0.5]),
            },
            Tag::Data {
                kind: DataTagKind::FreeBlock,
                data: Data::String("free blocks".into()),
            },
        ]
    }

    fn default_results() -> ResultSet {
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
