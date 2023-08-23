use meginfo;

#[test]
fn can_read_tags() {
    let parser = meginfo::parser::FifParser::new(vec![]);

    let tags = parser
        .read_tags("data/file_0.fif".into())
        .expect("Should have been able to read tags from test file");

    assert_eq!(tags.len(), 679);
}
