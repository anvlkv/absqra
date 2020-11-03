use super::*;
use ra_dev_tools::insta::{assert_json_snapshot, assert_snapshot};
use ra_dev_tools::make_example_tests;
use ra_tree_parser::parser::parse;
use ra_lexer::tokenize;
use std::convert::TryFrom;
use std::panic::catch_unwind;

#[make_example_tests]
#[test]
fn it_should_match_snapshots(contents: String, file_name: String) {
    let tree = parse(tokenize(&contents));

    match RaAST::try_from(tree.unwrap()) {
        Ok(ast) => assert_json_snapshot!(file_name, ast),
        Err((errors, parsed)) => {
            let formatted_errors: String = {
                errors.into_iter().map(|e| format!("{}", e)).collect::<Vec<String>>().join("\n")
            };

            let _ = catch_unwind(|| {
                assert_snapshot!(format!("{}__{}",file_name, "ERR__MESSAGES"), formatted_errors);
            });

            let _ = catch_unwind(|| {
                assert_json_snapshot!(format!("{}__{}", file_name, "ERR"), parsed);
            });


            panic!("failed to parse example {:?}", file_name);
        }
    }
}