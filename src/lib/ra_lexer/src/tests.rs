use crate::cursor::{Cursor, Position};
use crate::{tokenize, Token, TokenKind};

mod cursor {
    use super::{Cursor, Position};
    #[test]
    fn it_should_create() {
        Cursor::new("abc", Position(1, 0), 0, 0);
    }

    #[test]
    fn it_should_give_first_char() {
        let cur = Cursor::new("abc", Position(1, 0), 0, 0);
        assert_eq!(cur.first_ahead(), 'a');
    }

    #[test]
    fn it_should_give_second_char() {
        let cur = Cursor::new("abc", Position(1, 0), 0, 0);
        assert_eq!(cur.second_ahead(), 'b');
    }

    #[test]
    fn it_should_return_next_char() {
        let mut cur = Cursor::new("a", Position(1, 0), 0, 0);
        assert_eq!(cur.bump().unwrap(), 'a');
    }

    #[test]
    fn it_should_return_none_at_end_of_input() {
        let mut cur = Cursor::new("a", Position(1, 0), 0, 0);
        cur.bump();
        assert_eq!(cur.bump(), None);
    }

    #[test]
    fn it_should_confirm_eof() {
        let mut cur = Cursor::new("a", Position(1, 0), 0, 0);
        cur.bump();
        cur.bump();
        cur.bump();
        assert_eq!(cur.is_eof(), true);
    }

    #[test]
    fn it_should_return_amount_of_consumed_symbols() {
        let mut cur = Cursor::new("abc", Position(1, 0), 0, 0);
        cur.bump();
        cur.bump();
        cur.bump();
        assert_eq!(cur.len_consumed(), 3);
    }

    #[test]
    fn it_should_not_panic_when_encountering_funny_characters() {
        let mut cur = Cursor::new("🚬", Position(1, 0), 0, 0);
        assert_eq!(cur.bump().unwrap(), '🚬');
    }

    #[test]
    fn it_should_track_position() {
        let mut cur = Cursor::new("abc\nSOME", Position(1, 0), 0, 0);
        assert_eq!(cur.bump().unwrap(), 'a');
        assert_eq!(cur.position, Position(1, 1));
        assert_eq!(cur.bump().unwrap(), 'b');
        assert_eq!(cur.position, Position(1, 2));
        assert_eq!(cur.bump().unwrap(), 'c');
        assert_eq!(cur.position, Position(1, 3));
        assert_eq!(cur.bump().unwrap(), 'S');
        assert_eq!(cur.position, Position(2, 1));
        assert_eq!(cur.bump().unwrap(), 'O');
        assert_eq!(cur.position, Position(2, 2));
        assert_eq!(cur.bump().unwrap(), 'M');
        assert_eq!(cur.position, Position(2, 3));
    }

    #[test]
    fn it_should_track_indent_level() {
        let mut cur = Cursor::new("a\n\tb\n\t\tc\nd", Position(1, 0), 0, 0);
        assert_eq!(cur.bump().unwrap(), 'a');
        assert_eq!(cur.level, 0);
        assert_eq!(cur.bump().unwrap(), 'b');
        assert_eq!(cur.level, 1);
        assert_eq!(cur.bump().unwrap(), 'c');
        assert_eq!(cur.level, 2);
        assert_eq!(cur.position, Position(3, 3));
        assert_eq!(cur.bump().unwrap(), 'd');
        assert_eq!(cur.level, 0);
    }
}

mod lib {
    use super::{tokenize, Position, Token, TokenKind};
    

    #[test]
    fn it_should_create_iterator_of_tokens() {
        let mut stream = tokenize("abc");
        stream.next();
    }

    #[test]
    fn it_should_parse_multiple_lines() {
        let mut stream = tokenize("!?&/\n\tabc");
        stream.next();
        stream.next();
        stream.next();
        stream.next();
        assert_eq!(
            stream.next().unwrap(),
            Token {
                kind: Some(TokenKind::Identifier("abc")),
                content: "abc",
                position: (Position(2, 1), Position(2, 4)),
                len: 3,
                level: 1,
            }
        )
    }

    #[test]
    fn it_should_parse_identifiers() {
        let mut stream = tokenize("abc");
        assert_eq!(
            stream.next().unwrap(),
            Token {
                kind: Some(TokenKind::Identifier("abc")),
                content: "abc",
                position: (Position(1, 0), Position(1, 3)),
                len: 3,
                level: 0
            }
        );
    }

    use ra_common::insta::assert_debug_snapshot;
    use utils::for_each_ra_example_file;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn it_should_match_snapshots() {
        for_each_ra_example_file(|example| {
                let mut file = File::open(example.path()).unwrap();
    
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();

                println!("{}", example.file_name().to_str().unwrap());

                let tokens: Vec<Token> = tokenize(&contents).collect();
                assert_debug_snapshot!(example.path().to_str().unwrap(), tokens)
        });
    }

    mod strings {
        use super::{tokenize, Position, Token, TokenKind};
        #[test]
        fn it_should_parse_string_literals_with_double_quotes() {
            let mut stream = tokenize("\"some\"");
            assert_eq!(
                stream.next().unwrap(),
                Token {
                    kind: Some(TokenKind::StringLiteral("some")),
                    content: "some",
                    len: 6,
                    position: (Position(1, 0), Position(1, 6)),
                    ..Default::default()
                }
            )
        }

        #[test]
        fn it_should_parse_string_literals_with_single_quotes() {
            let mut stream = tokenize("\'some\'");
            assert_eq!(
                stream.next().unwrap(),
                Token {
                    kind: Some(TokenKind::StringLiteral("some")),
                    content: "some",
                    len: 6,
                    position: (Position(1, 0), Position(1, 6)),
                    ..Default::default()
                }
            )
        }
    }
    mod comments {
        use super::{tokenize, Position, Token, TokenKind};
        #[test]
        fn it_should_parse_single_line_comments() {
            let mut stream = tokenize("//abc");
            assert_eq!(
                stream.next().unwrap(),
                Token {
                    kind: Some(TokenKind::Comment),
                    content: "abc",
                    position: (Position(1, 0), Position(1, 5)),
                    len: 5,
                    level: 0
                }
            );
        }
        #[test]
        fn it_should_parse_multi_line_comments() {
            let mut stream = tokenize("/*abc\nSOME*/");
            assert_eq!(
                stream.next().unwrap(),
                Token {
                    kind: Some(TokenKind::Comment),
                    content: "abc\nSOME",
                    position: (Position(1, 0), Position(2, 6)),
                    len: 12,
                    level: 0
                }
            );
        }

        #[test]
        fn it_should_parse_multi_line_comments_with_fringe() {
            let mut stream = tokenize("/*\n* abc\n* SOME\n*/");
            assert_eq!(
                stream.next().unwrap(),
                Token {
                    kind: Some(TokenKind::Comment),
                    content: "\n* abc\n* SOME\n",
                    position: (Position(1, 0), Position(4, 2)),
                    len: 18,
                    level: 0
                }
            );
        }

        #[test]
        fn it_should_parse_comments_followed_by_one_another() {
            let mut stream = tokenize("//some\n\n/*\n* abc\n* SOME\n*/");
            assert_eq!(
                stream.next().unwrap(),
                Token {
                    kind: Some(TokenKind::Comment),
                    content: "some",
                    position: (Position(1, 0), Position(1, 6)),
                    len: 6,
                    level: 0
                }
            );

            assert_eq!(
                stream.next().unwrap(),
                Token {
                    kind: Some(TokenKind::Comment),
                    content: "\n* abc\n* SOME\n",
                    position: (Position(3, 0), Position(6, 2)),
                    len: 18,
                    level: 0
                }
            );
        }
    }
    
    mod numbers {
        use super::{tokenize, Position, Token, TokenKind};
        #[test]
        fn it_should_parse_numbers() {
            let mut stream = tokenize("123");
            assert_eq!(
                stream.next().unwrap(),
                Token {
                    kind: Some(TokenKind::Int(123)),
                    content: "123",
                    position: (Position(1, 0), Position(1, 3)),
                    len: 3,
                    level: 0
                }
            );
        }

        #[test]
        fn it_should_parse_numbers_with_decimal_separator() {
            let mut stream = tokenize("123,321");
            assert_eq!(
                stream.next().unwrap(),
                Token {
                    kind: Some(TokenKind::Float(123.321)),
                    content: "123,321",
                    position: (Position(1, 0), Position(1, 7)),
                    len: 7,
                    level: 0
                }
            );
        }

        #[test]
        fn it_should_parse_numbers_with_another_decimal_separator() {
            let mut stream = tokenize("123.321");
            assert_eq!(
                stream.next().unwrap(),
                Token {
                    kind: Some(TokenKind::Float(123.321)),
                    content: "123.321",
                    position: (Position(1, 0), Position(1, 7)),
                    len: 7,
                    level: 0
                }
            );
        }

        #[test]
        fn it_should_parse_numbers_with_decimal_and_thousands_separator() {
            let mut stream = tokenize("123.321,456");
            assert_eq!(
                stream.next().unwrap(),
                Token {
                    kind: Some(TokenKind::Float(123321.456)),
                    content: "123.321,456",
                    position: (Position(1, 0), Position(1, 11)),
                    len: 11,
                    level: 0
                }
            );
        }

        #[test]
        fn it_should_parse_numbers_with_decimal_and_multiple_thousands_separator() {
            let mut stream = tokenize("123.321.123,456");
            assert_eq!(
                stream.next().unwrap(),
                Token {
                    kind: Some(TokenKind::Float(123321123.456)),
                    content: "123.321.123,456",
                    position: (Position(1, 0), Position(1, 15)),
                    len: 15,
                    level: 0
                }
            );
        }

        #[test]
        fn it_should_parse_numbers_with_another_decimal_and_multiple_thousands_separators() {
            let mut stream = tokenize("123,321,123.456");
            assert_eq!(
                stream.next().unwrap(),
                Token {
                    kind: Some(TokenKind::Float(123321123.456)),
                    content: "123,321,123.456",
                    position: (Position(1, 0), Position(1, 15)),
                    len: 15,
                    level: 0
                }
            );
        }

        #[test]
        #[should_panic]
        fn it_should_panic_when_encountering_multiple_decimal_separators() {
            let mut stream = tokenize("123.321.123,456,654");
            stream.next();
        }

        #[test]
        fn it_should_parse_negative_integer() {
            let mut stream = tokenize("-123");
            assert_eq!(
                stream.next().unwrap(),
                Token {
                    kind: Some(TokenKind::Int(-123)),
                    content: "-123",
                    position: (Position(1, 0), Position(1, 4)),
                    len: 4,
                    level: 0
                }
            )
        }

        #[test]
        fn it_should_parse_negative_float() {
            let mut stream = tokenize("-123.312");
            assert_eq!(
                stream.next().unwrap(),
                Token {
                    kind: Some(TokenKind::Float(-123.312)),
                    content: "-123.312",
                    position: (Position(1, 0), Position(1, 8)),
                    len: 8,
                    level: 0
                }
            )
        }
    }

    mod content_block {
        use super::{tokenize, Position, Token, TokenKind};
        #[test]
        fn it_should_parse_content_blocks() {
            let mut stream = tokenize("`abc`");
            assert_eq!(
                stream.next().unwrap(),
                Token {
                    kind: Some(TokenKind::ContentBlock),
                    content: "abc",
                    position: (Position(1, 0), Position(1, 5)),
                    len: 5,
                    level: 0
                }
            );
        }

        #[test]
        fn it_should_keep_inner_indents_when_parsing_content() {
            let mut stream = tokenize("`\n\tabc\n\t\tabc\n`");
            assert_eq!(
                stream.next().unwrap(),
                Token {
                    kind: Some(TokenKind::ContentBlock),
                    content: "\n\tabc\n\t\tabc\n",
                    position: (Position(1, 0), Position(4, 1)),
                    len: 14,
                    level: 0
                }
            );
        }

        #[test]
        #[should_panic]
        fn it_should_panic_when_content_block_is_not_closed_properly() {
            let mut stream = tokenize("`\n\tabc`");
            stream.next();
        }
    }

    mod single_char_tokens {
        use super::{tokenize, Position, Token, TokenKind};
        #[test]
        fn it_should_parse_tokens() {
            let mut stream = tokenize("!?&/#");
            assert_eq!(
                stream.next().unwrap(),
                Token {
                    kind: Some(TokenKind::Exclamation),
                    position: (Position(1, 0), Position(1, 1)),
                    content: "!",
                    level: 0,
                    ..Default::default()
                }
            );
            assert_eq!(
                stream.next().unwrap(),
                Token {
                    kind: Some(TokenKind::Question),
                    position: (Position(1, 1), Position(1, 2)),
                    content: "?",
                    level: 0,
                    ..Default::default()
                }
            );
            assert_eq!(
                stream.next().unwrap(),
                Token {
                    kind: Some(TokenKind::Ampersand),
                    position: (Position(1, 2), Position(1, 3)),
                    content: "&",
                    level: 0,
                    ..Default::default()
                }
            );
            assert_eq!(
                stream.next().unwrap(),
                Token {
                    kind: Some(TokenKind::Slash),
                    position: (Position(1, 3), Position(1, 4)),
                    content: "/",
                    level: 0,
                    ..Default::default()
                }
            );
            assert_eq!(
                stream.next().unwrap(),
                Token {
                    kind: Some(TokenKind::HashPound),
                    position: (Position(1, 4), Position(1, 5)),
                    content: "#",
                    level: 0,
                    ..Default::default()
                }
            );
        }
    }


}
