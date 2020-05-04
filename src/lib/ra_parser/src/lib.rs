mod block;
mod cursor;
mod errors;
mod expression;
// // pub mod expression;

use ra_lexer::token::Token;
use ra_lexer::tokenize;

use block::{Block, BlockKind};
// use std::fmt;
// use std::convert::TryInto;
use cursor::Cursor;
use errors::ParserError;
// // use expression::{OutputExpression, InputExpression};

// impl <'a> Block<'a> {
//     pub fn new(kind: BlockKind, level: u16) -> Block {
//         Block {
//             kind,
//             children: Vec::new(),
//             level
//         }
//     }
// }

// impl <'a> fmt::Debug for Block<'a> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         // format!(, , "*indented* text");
//         write!(
//             f, "\n{}{}\n{}{}\n{}{}" ,
//             format_args!("{: >1$}", "", self.level.try_into().unwrap()),
//             format_args!(
//                 "L{:?} [{:?}_Block", self.level ,self.kind
//             ),
//             // format_args!("{: >1$}", "", self.level),
//             // format_args!(
//             //     "expr:{:?}", self.expression
//             // ),
//             format_args!("{: >1$}", "", self.level.try_into().unwrap()),
//             format_args!(
//                 "children:{:?}", self.children
//             ),
//             format_args!("{: >1$}", "", self.level.try_into().unwrap()),
//             format_args!(
//                 "]"
//             )
//         )
//     }
// }

pub fn parse<'a>(input: &'a str) -> Result<Block<'a>, Vec<ParserError>> {
    let stream = tokenize(input);
    let mut errors: Vec<ParserError> = Vec::new();
    
    let program = {
        let mut children = Vec::new();
        let mut tokens_cursor = Cursor::new(stream);
        
        loop {
            if !tokens_cursor.is_eof() {
                match tokens_cursor.advance_block() {
                    Ok(blk) => children.push(blk),
                    Err(es) => errors.extend(es.into_iter()),
                }
            }
            else {
                break;
            }
        }

        Block {
            children,
            level: 0,
            kind: BlockKind::Program,
            ..Default::default()
        }
    };

    if errors.len() > 0 {
        Err(errors)
    } else {
        Ok(program)
    }
}

impl<'token, I> Cursor<'token, I>
where
    I: Iterator<Item = Token<'token>>,
{
    fn advance_block<'a>(&'a mut self) -> Result<Block<'token>, Vec<ParserError>> {
        let first_token = self.bump();
        let mut errors = Vec::new();
        match first_token {
            Some(token) => {

                Ok(Block::default())
            },
            None => {
                errors.push(ParserError::UnexpectedEndOfInput);
                Err(errors)
            }
        }
    }

    
}

// pub fn parse<'parser, 'runtime> (input: &'runtime str) -> Block<'runtime> {
//     let stream = tokenize(input);
//     // println!("{:?}", input);
//     let mut cursor = Cursor::new(stream);

//     // println!("{:?}", cursor.first_ahead());

//     let mut block = Block::new(BlockKind::Program, 0);

//     while !cursor.is_eof() {
//         let b = cursor.advance_block();
//         match b {
//             Ok(blk) => block.children.push(blk),
//             Err(e) => panic!("{:?}", e)
//         }
//     }

//     block
// }

// impl <'a> Cursor<'a> {
//     fn advance_block(&mut self) -> Result<Block<'a>, ParserError> {
//         let mut current_block_result = match self.bump() {
//             Some(t) => {
//                 match t.kind {
//                     TokenKind::Identifier(_) |
//                     TokenKind::Float(_) |
//                     TokenKind::Int(_) |
//                     TokenKind::OpenParentheses => {
//                         self.parse_output_block(t)
//                     },
//                     TokenKind::Exclamation => {
//                         self.parse_rule_invocation_block(t)
//                     },
//                     TokenKind::Plus |
//                      TokenKind::Greater => {
//                         self.parse_input_block(t)
//                     },
//                     TokenKind::Colon => {
//                         self.parse_rule_declaration_block(t)
//                     },
//                     TokenKind::OpenCurlyBrace => {
//                         self.parse_context_declaration(t)
//                     },
//                     TokenKind::ContentBlock => {
//                         self.parse_content_block(t)
//                     },
//                     TokenKind::At =>  {
//                         self.parse_reference_block(t)
//                     },
//                     TokenKind::Comment => {
//                         self.advance_block()
//                     },
//                     _ => return Err(ParserError::UnexpectedToken(t))
//                 }
//             }
//             None => return Err(ParserError::UnexpectedEndOfInput)
//         };

//         if let Ok(current_block) = &mut current_block_result {
//             let next_token = self.first_ahead();
//             if let Some(tok) = next_token {
//                 if tok.level > current_block.level {
//                     match self.parse_children() {
//                         Ok(children) => current_block.children = children,
//                         Err(e) => return Err(e)
//                     }
//                 }
//                 else if tok.kind == TokenKind::Coma {
//                     return self.parse_union_block(current_block.clone())
//                 }
//             }
//         }

//         current_block_result
//     }

//     fn parse_input_block(&mut self, first_token: Token) -> Result<Block, ParserError> {
//         let is_expansive = match &first_token.kind {
//             TokenKind::Plus => {
//                 self.bump();
//                 true
//             },
//             TokenKind::Greater => {
//                 false
//             },
//             _ => return Err(ParserError::UnexpectedToken(first_token))
//         };

//         // let mut level_position_constrain = |level, position: Position, _| level == first_token.level && position.0 == (first_token.position.0).0;

//         // self.match_while(level_position_constrain, |token| {
//         // })

//         let mut block = Block::new(BlockKind::Input(is_expansive, InputExpression::default()), first_token.level);

//         // let block_tokens = self.match_while();
//         // block.expression = parse_expression(block_tokens);

//         Ok(block)
//     }

//     fn parse_output_block(&mut self, first_token: Token) -> Result<Block<'a>, ParserError> {
//         let mut block = Block::new(BlockKind::Undetermined, first_token.level);
//         let mut block_tokens: Vec<Token> = Vec::new();
//         let output_token = match self.first_ahead() {
//             Some(next_token) => {
//                 if next_token.level == first_token.level && (next_token.position.0).0 == (first_token.position.0).0 {
//                     block_tokens.push(first_token.clone());
//                     Token {
//                         kind: TokenKind::Immediate,
//                         level: first_token.level,
//                         len: 0,
//                         position: (first_token.position.0, first_token.position.0),
//                         ..Default::default()
//                     }
//                 }
//                 else {
//                     first_token
//                 }
//             },
//             None => first_token
//         };

//         let mut output_expression = OutputExpression::default();
//         self.do_while(
//             |level, position, _| level == output_token.level && position.0 == (output_token.position.0).0,
//             |token| output_expression.append_token(token).unwrap()
//         );

//         block.kind = BlockKind::Output(output_token.clone(), output_expression);

//         Ok(block)
//     }

//     fn parse_reference_block(&mut self, first_token: Token) -> Result<Block, ParserError> {
//         let mut block = Block::new(BlockKind::Reference, first_token.level);
//         // let block_tokens = self.do_while(|level, position, _| level == first_token.level && position.0 == (first_token.position.0).0);
//         // block.expression = parse_expression(block_tokens);
//         Ok(block)
//     }

//     fn parse_rule_declaration_block(&mut self, first_token: Token) -> Result<Block, ParserError> {
//         let mut block = Block::new(BlockKind::Undetermined, first_token.level);
//         // let mut block_tokens = self.do_while(|level, position, _| level == first_token.level && position.0 == (first_token.position.0).0);
//         // block.kind = BlockKind::RuleDeclaration(block_tokens.remove(0));
//         // block.expression = parse_expression(block_tokens);
//         Ok(block)
//     }

//     fn parse_rule_invocation_block(&mut self, first_token: Token) -> Result<Block, ParserError> {
//         let mut block = Block::new(BlockKind::Undetermined, first_token.level);
//         // let mut block_tokens = self.do_while(|level, position, _| level == first_token.level && position.0 == (first_token.position.0).0);
//         // block.kind = BlockKind::RuleInvocation(block_tokens.remove(0));
//         Ok(block)
//     }

//     fn parse_context_declaration(&mut self, first_token: Token) -> Result<Block, ParserError> {
//         let mut block = Block::new(BlockKind::Undetermined/*ContextDeclaration*/, first_token.level);
//         // let block_tokens = self.read_within(first_token.kind, TokenKind::CloseCurlyBrace);
//         // block.expression = parse_expression(block_tokens);
//         self.bump();
//         Ok(block)
//     }

//     fn parse_content_block(&mut self, content_token: Token) -> Result<Block, ParserError> {
//         let mut block = Block::new(BlockKind::Undetermined/*Content*/, content_token.level);
//         // block.expression = parse_expression(init_tokens);
//         // let mut block_tokens = vec!content_token);
//         Ok(block)
//     }

//     fn parse_union_block(&mut self, first_block: Block) -> Result<Block<'a>, ParserError> {
//         self.bump();
//         let mut block = Block::new(BlockKind::Union(1), first_block.level);
//         block.children.push(first_block);
//         loop {
//             let next_block = self.advance_block();
//             match next_block {
//                 Ok(b) => {
//                     block.children.push(b);
//                     block.kind = BlockKind::Union(block.children.len())
//                 }
//                 Err(e) => return Err(e)
//             }
//             match self.first_ahead() {
//                 Some(t) => {
//                     if t.kind != TokenKind::Coma {
//                         break;
//                     }
//                 }
//                 None => break
//             }
//         }

//         Ok(block)
//     }

//     fn parse_children(&mut self) -> Result<Vec<Block>, ParserError> {
//         let mut children = Vec::<Block>::new();
//         let initial_level = self.level;

//         loop {
//             let next_token = self.first_ahead();
//             match next_token {
//                 Some(tok) => {
//                     if tok.level > initial_level {
//                         match self.advance_block() {
//                             Ok(b) => children.push(b),
//                             Err(e) => return Err(e)
//                         }
//                     }
//                     else {
//                         break
//                     }
//                 }
//                 None => break
//             }
//         }

//         Ok(children)
//     }
// }

// // fn parse_block(mut block: Block) -> Block {
// //     let mut tokens = block.tokens.into_iter();

// //     while let Some(token) = tokens.next() {
// //         let mut current_block = Block::new(BlockKind::Undetermined, token.level);

// //         if token.level == block.level {
// //             match token.kind {
// //
// //                 _ => {
// //                     panic!(ParserError::UnexpectedToken);
// //                 }
// //             }
// //         }
// //         else if token.level > block.level {
// //             let first_token_level = token.level;
// //             let mut child_block = Block::new(BlockKind::Undetermined, first_token_level);
// //             child_block.tokens.push(token);
// //             loop {
// //                 match tokens.nth(0) {
// //                     Some(t) => {
// //                         if t.level > first_token_level {
// //                             child_block.tokens.push(tokens.next().unwrap());
// //                         }
// //                         else {
// //                             break;
// //                         }
// //                     }
// //                     None => break
// //                 }
// //             }
// //             thread::spawn(move || {
// //                 current_block.children.push(parse_block(child_block))
// //             });
// //         }
// //     }

// //     block
// // }

// // fn old_parse_block(mut block: Block) -> Block {
// //     let block_data = block.clone();
// //     let block_level = block_data.level;
// //     let mut block_level_tokens = block_data.tokens.clone().into_iter().filter(|tok| tok.level == block_level);
// //     println!("{:?}", block_level_tokens);
// //     println!("{:?}", block.tokens);
// //     while let Some(start_token) = block_level_tokens.next() {
// //         assert_eq!(&start_token, &block.tokens.remove(0));

// //         let mut current_block = Block::new(BlockKind::Undetermined, start_token.level);
// //         // let mut last_token_index = block_data.tokens.clone().iter().position(|tok| tok == &start_token).unwrap();

// //         current_block.tokens.push(start_token.clone());

// //         loop {
// //             let next = block_level_tokens.nth(0);
// //             match next {
// //                 Some(next_token) => {
// //                     if (next_token.position.0).0 == (start_token.position.0).0 {
// //                         current_block.tokens.push(block_level_tokens.next().unwrap());
// //                         // last_token_index += 1;
// //                         block.tokens.remove(0);
// //                     }
// //                     else {
// //                         break;
// //                     }
// //                 },
// //                 None => break
// //             }
// //         }

// //         if block.tokens.len() != 0 && block.tokens[0].level > current_block.level {
// //             let child_block_level = block.tokens[0].level;
// //             loop {
// //                 let mut child_block = Block::new(BlockKind::Undetermined, child_block_level);
// //                 loop {
// //                     child_block.tokens.push(block.tokens.remove(0));
// //                     if block.tokens[0].level <= current_block.level {
// //                         break;
// //                     }
// //                 }
// //                 current_block.children.push(parse_block(child_block));
// //                 if block.tokens[0].level < child_block_level {
// //                     break;
// //                 }
// //             }
// //         }

// //         block.children.push(current_block);
// //     }
// //     block
// // }

#[cfg(test)]
mod tests {
    use super::expression::{Expression, ExpressionMember, OperationKind};
    use super::{parse, BlockKind};
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
                kind: TokenKind::Identifier("abc"),
                len: 3,
                level: 0,
                position: (Position(1, 0), Position(1, 3))
            })
        );
        assert_eq!(
            parsed.expression.right.as_ref().unwrap().as_ref(),
            &ExpressionMember::Token(Token {
                content: "2",
                kind: TokenKind::Int(2),
                len: 1,
                level: 0,
                position: (Position(1, 6), Position(1, 7))
            })
        );
    }
}
