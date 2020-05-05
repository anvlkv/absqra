mod cursor;
pub mod block;
pub mod errors;
pub mod expression;
pub mod parser;

#[cfg(test)]
mod tests {
    use super::expression::{Expression, ExpressionMember, OperationKind};
    use super::parser::parse;
    use super::block::BlockKind;

    use ra_lexer::cursor::Position;
    use ra_lexer::token::{Token, TokenKind};

    #[test]
    fn it_should_return_program_block() {
        assert_eq!(parse("abc").expect("can't parse").kind, BlockKind::Program);
    }

    #[test]
    fn it_should_parse_block_level_expressions() {
        let program = parse("abc + 2").expect("can't parse");
        let parsed = program.children.iter().nth(0).unwrap();
        assert_eq!(parsed.expression.operation, OperationKind::Sum);
        assert_eq!(
            parsed.expression.left.as_ref().unwrap().as_ref(),
            &ExpressionMember::Token(Token {
                content: "abc",
                kind: Some(TokenKind::Identifier("abc")),
                len: 3,
                level: 0,
                position: (Position(1, 0), Position(1, 3))
            })
        );
        assert_eq!(
            parsed.expression.right.as_ref().unwrap().as_ref(),
            &ExpressionMember::Token(Token {
                content: "2",
                kind: Some(TokenKind::Int(2)),
                len: 1,
                level: 0,
                position: (Position(1, 6), Position(1, 7))
            })
        );
    }
}
