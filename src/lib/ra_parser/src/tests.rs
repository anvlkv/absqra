mod lib {
    use ra_lexer::cursor::Position;
    use ra_lexer::token::{Token, TokenKind};

    use crate::block::BlockKind;
    use crate::expressions::output_expression::ExpressionMember;
    use crate::expressions::output_expression::{MathOperation, OperationKind, OutputExpression};
    use crate::expressions::reference_expression::ReferenceExpression;
    use crate::parser::parse;

    #[test]
    fn it_should_return_program_block() {
        assert_eq!(parse("abc").expect("can't parse").kind, BlockKind::Program);
    }

    #[test]
    fn it_should_parse_block_level_expressions() {
        let program = parse("abc + 2").expect("can't parse");
        let parsed = program.children.iter().nth(0).unwrap();
        assert_eq!(
            parsed.kind,
            BlockKind::Output(OutputExpression(
                Box::new(ExpressionMember::ReferenceExpression(ReferenceExpression(Token {
                    kind: Some(TokenKind::Identifier("abc")),
                    position: (Position(1, 0), Position(1, 3)),
                    len: 3,
                    content: "abc",
                    level: 0,
                }, None))),
                Some(OperationKind::MathOperation(MathOperation::Sum)),
                Some(Box::new(ExpressionMember::Literal(Token {
                    kind: Some(TokenKind::Int(2)),
                    position: (Position(1, 6), Position(1, 7)),
                    level: 0,
                    content: "2",
                    len: 1
                })))
            ))
        )
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

    use ra_dev_tools::insta::{assert_json_snapshot, assert_snapshot};
    
    use std::fs::File;
    use std::io::Read;
    use ra_dev_tools::for_each_ra_example_file;

    #[test]
    fn it_should_match_snapshots() {
        println!("{:?}", std::env::args());
        for_each_ra_example_file(|example| {
            let mut file = File::open(example.path()).unwrap();
            
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();

            let block_tree = {
                match parse(&contents) {
                    Ok(b) => b,
                    Err((errors, parsed)) => {

                        let formatted_errors: String = {
                            errors.into_iter().map(|e| format!("{}", e)).collect::<Vec<String>>().join("\n")
                        };

                        
                        assert_snapshot!(String::from(example.path().to_str().unwrap()) + "__ERR__MESSAGES__", formatted_errors);
                        assert_json_snapshot!(String::from(example.path().to_str().unwrap()) + "__ERR__", parsed);


                        panic!("failed to parse example {:?}", example.path());
                    }
                }
            };
            assert_json_snapshot!(example.path().to_str().unwrap(), block_tree)
        })       
    }
}
