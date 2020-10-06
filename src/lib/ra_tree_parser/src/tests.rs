mod lib {
    use ra_dev_tools::insta::{assert_json_snapshot, assert_snapshot};    
    use ra_dev_tools::{make_example_tests};
    use ra_lexer::tokenize;
    use crate::parser::parse;
    use std::fs::remove_file;
    use std::path::{PathBuf};
    use std::env::current_dir;

    #[make_example_tests]
    #[test]
    fn it_should_match_snapshots(contents: String, file_name: String) {
        let err_messages_file_name = format!("{}__{}",file_name, "ERR__MESSAGES");
        let errors_file_name = format!("{}__{}", file_name, "ERR");
        let tree = {
            let mut tokens_iter = tokenize(&contents);
            match parse(tokens_iter) {
                Ok(tree) => {
                    tree
                },
                Err((errors, parsed)) => {
                    let formatted_errors: String = {
                        errors.into_iter().map(|e| format!("{}", e)).collect::<Vec<String>>().join("\n")
                    };

                    
                    assert_snapshot!(err_messages_file_name, formatted_errors);
                    assert_json_snapshot!(errors_file_name, parsed);


                    panic!("failed to parse example {:?}", file_name);
                }
            }
        };
        assert_json_snapshot!(file_name, tree)       
    }
}
