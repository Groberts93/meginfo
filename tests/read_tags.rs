use meginfo;
use std::fs::File;

#[test]
fn can_read_tags() {
    let parser = meginfo::parser::FifParser::new(vec![]);

    let fh = File::open("data/file_0.fif").expect("Should have been a test file here");
    let tags = parser
        .read_tags(fh)
        .expect("Should have been able to read tags from test file");

    assert_eq!(tags.len(), 679);
}
