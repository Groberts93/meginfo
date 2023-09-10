use fiff::parser::FifParser;

#[test]
pub fn can_read_tags() {
    let tags = FifParser::read_tags("data/file_0.fif".into())
        .expect("Should have been able to read tags from test file");

    assert_eq!(tags.len(), 671);
}
