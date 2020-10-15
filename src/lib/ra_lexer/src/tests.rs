use super::*;
use ra_dev_tools::insta::assert_json_snapshot;
use ra_dev_tools::make_example_tests;

#[make_example_tests]
#[test]
fn it_should_match_snapshots(contents: String, file_name: String) {
    let tokens: Vec<RaToken> = tokenize(&contents).map(|r| r.unwrap()).collect();
    assert_json_snapshot!(file_name, tokens)
}

#[test]
fn it_should_tokenize_special_characters() {
    let contents = String::from("!@#$%^&*()_+=-{}?:;\\/|~");
    let tokens: Vec<RaToken> = tokenize(&contents).map(|r| r.unwrap()).collect();
    assert_json_snapshot!("special_characters", tokens)
}