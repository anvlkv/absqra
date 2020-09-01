mod lib {
    use ra_lexer::cursor::Position;
    use ra_lexer::token::{RaToken, TokenKind};
    use ra_dev_tools::insta::{assert_json_snapshot, assert_snapshot};    
    use std::fs::{File, DirEntry};
    use std::io::Read;
    use ra_dev_tools::{make_example_tests};
    use crate::parser::parse;

    


    #[make_example_tests]
    #[test]
    fn it_should_match_snapshots(contents: String, file_name: String) {
        let block_tree = {
            match parse(&contents) {
                Ok(b) => b,
                Err((errors, parsed)) => {

                    let formatted_errors: String = {
                        errors.into_iter().map(|e| format!("{}", e)).collect::<Vec<String>>().join("\n")
                    };

                    
                    assert_snapshot!(format!("{}__{}",file_name, "ERR__MESSAGES"), formatted_errors);
                    assert_json_snapshot!(format!("{}__{}", file_name, "ERR"), parsed);


                    panic!("failed to parse example {:?}", file_name);
                }
            }
        };
        assert_json_snapshot!(file_name, block_tree)       
    }
}
