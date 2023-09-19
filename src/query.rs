//! Higher level API for searching multiple .fif files for list of tags.

use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    path::PathBuf,
};

use crate::FifParser;
use anyhow::Result;
use log::info;

use crate::{
    enums::DataTagKind,
    tag::{Data, LabelledData, Tag},
};

type QuerySet = HashSet<DataTagKind>;
type ResultSet = HashMap<DataTagKind, Vec<Data>>;

#[derive(Debug)]
pub struct Search {
    orders: (Vec<PathBuf>, Vec<DataTagKind>),
    query: QuerySet,
    state: HashMap<PathBuf, SearchState>,
}

impl Search {
    pub fn new(codes: Vec<DataTagKind>, files: Vec<PathBuf>) -> Self {
        let mut state: HashMap<PathBuf, SearchState> = HashMap::new();

        for file in files.iter() {
            state.insert(file.clone(), SearchState::Pending);
        }

        Search {
            orders: (files.clone(), codes.clone()),
            query: HashSet::from_iter(codes.into_iter()),
            state: state,
        }
    }

    pub fn execute(&mut self) {
        let query = self.query.clone();

        for (file, state) in self.state.iter_mut() {
            *state = SearchState::Complete(Self::search_tags(file.clone(), query.clone()).unwrap());
        }
    }

    fn search_tags(file: PathBuf, query: QuerySet) -> Result<ResultSet> {
        let tags = FifParser::read_tags(file)?;

        let mut results = ResultSet::new();

        for tag in tags {
            if let Tag::Data { kind, data } = tag {
                if query.contains(&kind) {
                    results
                        .entry(kind)
                        .and_modify(|x| x.push(data.clone()))
                        .or_insert(vec![data]);
                }
            }
        }

        for (kind, tvec) in results.iter() {
            if tvec.len() > 1 {
                info!("found {} tags for {:?}", tvec.len(), kind);
            }
        }

        Ok(results)
    }
}

impl Display for Search {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut wtr = csv::WriterBuilder::new()
            .delimiter(b',')
            .from_writer(vec![]);

        // write header
        wtr.serialize(("file", &self.orders.1)).unwrap();

        // write entries with file and queried tags in original order
        for file in self.orders.0.iter() {
            let result = self.state.get(file).expect("File should be in map");

            if let SearchState::Complete(results) = result {
                let mut output: Vec<String> = self
                    .orders
                    .1
                    .iter()
                    .map(|x| {
                        let kind = x.clone();
                        results
                            .get(x)
                            .unwrap_or(&vec![])
                            .get(0)
                            .map_or("Not found".to_owned(), |x| {
                                LabelledData::new(kind, x.clone()).to_string()
                            })
                    })
                    .collect();

                let mut filename = vec![String::from(file.file_name().unwrap().to_str().unwrap())];
                filename.append(&mut output);

                wtr.serialize(filename).unwrap();
            }
        }

        let data = String::from_utf8(wtr.into_inner().unwrap()).unwrap();
        write!(f, "{}", data)
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
        let search = Search::new(default_query(), default_files());

        let mut state = HashMap::new();

        for file in default_files() {
            state.insert(file, SearchState::Pending);
        }

        assert_eq!(search.state, state);
        assert_eq!(
            search.query,
            HashSet::from_iter(default_query().into_iter())
        );
    }

    #[test]
    fn can_execute_search() {
        // this requires default_files, default_tags, default_results, and default_query to be correct
        let mut search = Search::new(default_query(), default_files());
        search.execute();

        // TODO: actually check results: need to be able to serialize results structure

        println!("{search}");
    }

    fn default_files() -> Vec<PathBuf> {
        vec!["data/file_0.fif", "data/file_1.fif", "data/file_2.fif"]
            .iter()
            .map(|x| x.into())
            .collect()
    }

    fn default_query() -> Vec<DataTagKind> {
        vec![
            DataTagKind::FileId,
            DataTagKind::MeasDate,
            DataTagKind::Sfreq,
            DataTagKind::BadChs,
            DataTagKind::SphereLayers,
        ]
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
