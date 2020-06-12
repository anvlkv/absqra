mod lib {
    use ra_lexer::cursor::Position;
    use ra_lexer::token::{Token, TokenKind};

    use crate::expression::{Expression, OperationKind, MathOperation};
    use crate::parser::parse;
    use crate::block::BlockKind;

    
    #[test]
    fn it_should_return_program_block() {
        assert_eq!(parse("abc").expect("can't parse").kind, Some(BlockKind::Program));
    }

    #[test]
    fn it_should_parse_block_level_expressions() {
        let program = parse("abc + 2").expect("can't parse");
        let parsed = program.children.iter().nth(0).unwrap();
        // assert_eq!(parsed.expression.1, Some(OperationKind::MathOperation(MathOperation::Sum)));

        // assert_eq!(
        //     parsed.expression.0,
        //     Token {
        //         content: "abc",
        //         kind: Some(TokenKind::Identifier("abc")),
        //         len: 3,
        //         level: 0,
        //         position: (Position(1, 0), Position(1, 3))
        //     }
        // );

        // assert_eq!(
        //     (parsed.expression.2.as_ref().unwrap())
        //     Token {
        //         content: "2",
        //         kind: Some(TokenKind::Int(2)),
        //         len: 1,
        //         level: 0,
        //         position: (Position(1, 6), Position(1, 7))
        //     }
        // );
    }

    use insta::assert_debug_snapshot;
    use utils::files_from_dir_recursively;
    use std::ffi::OsStr;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn it_should_match_snapshots() {
        files_from_dir_recursively("../../../examples")
            .into_iter()
            .filter(|f| f.path().extension().and_then(OsStr::to_str).unwrap_or("") == "ra")
            .for_each(|example| {
                let mut file = File::open(example.path()).unwrap();
                println!("{}", example.file_name().to_str().unwrap());
    
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();


                let block_tree = parse(&contents).unwrap();
                assert_debug_snapshot!(example.path().to_str().unwrap(), block_tree)
        });
    }
}