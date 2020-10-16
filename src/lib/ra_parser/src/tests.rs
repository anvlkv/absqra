use super::*;
use ra_dev_tools::insta::assert_json_snapshot;
use ra_dev_tools::make_example_tests;
use ra_tree_parser::parser::parse;
use ra_lexer::tokenize;

#[make_example_tests]
#[test]
fn it_should_match_snapshots(contents: String, file_name: String) {
    let tree = parse(tokenize(&contents));
    let ast = RaAST::from(tree.unwrap());

    assert_json_snapshot!(file_name, ast)
}