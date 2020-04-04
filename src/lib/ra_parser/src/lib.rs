mod errors;
mod cursor;

use ra_lexer::{tokenize, Token, TokenKind};
use std::thread;
use errors::ParserError;
use cursor::Cursor;

#[derive(Clone, Debug)]
pub enum BlockKind {
    Program,
    Input(bool),
    Output(Token),
    RuleDeclaration(Token),
    RuleInvocation(Token),
    ContextDeclaration,
    Content,
    Undetermined,
}

#[derive(Clone, Debug)]
pub struct Block {
    kind: BlockKind,
    children: Vec<Block>,
    tokens: Vec<Token>,
    level: usize
}

impl Block {
    pub fn new(kind: BlockKind, level: usize) -> Block {
        Block {
            kind,
            children: Vec::new(),
            tokens: Vec::new(),
            level
        }
    }
}

pub fn parse(input: &str) {
    let stream = tokenize(input);
    // println!("{:?}", input);
    let mut cursor = Cursor::new(Box::new(stream));

    // println!("{:?}", cursor.first_ahead());

    while let Some(t) = cursor.advance_block() {
        println!("{:?}", t);
    }
}

impl Cursor<'_> {
    fn advance_block(&mut self) -> Option<Block> {
        let first_token = self.bump();
        let mut current_block = match first_token {
            Some(t) => {
                match t.clone().kind {
                    TokenKind::Identifier => {
                        self.parse_output_block(t)
                    },
                    TokenKind::Exclamation => {
                        self.parse_rule_invocation_block(t)
                    },
                    TokenKind::Plus => {
                        self.parse_input_block(t)
                    },
                    TokenKind::Colon => {
                        self.parse_rule_declaration_block(t)
                    },
                    TokenKind::OpenCurlyBrace => {
                        self.parse_context_declaration(t)
                    },
                    TokenKind::Greater => {
                        self.parse_input_block(t)
                    },
                    TokenKind::ContentBlock(init_tokens) => {
                        self.parse_content_block(t, init_tokens)
                    },
                    TokenKind::Comment => {
                        let next_expression = self.advance_block();
                        match next_expression {
                            Some(e) => e,
                            None => return None
                        }
                    },
                    _ => panic!(ParserError::UnexpectedToken)
                }
            }
            None => return None
        };

        let next_token = self.first_ahead();

        if let Some(tok) = next_token {
            if tok.level > current_block.level {
                current_block.children = self.parse_children()
            }
        }

        Some(current_block)
    }

    fn parse_input_block(&mut self, first_token: Token) -> Block {
        let is_expansive = match first_token.kind {
            TokenKind::Plus => {
                self.bump();
                true
            },
            TokenKind::Greater => {
                false
            },
            _ => panic!(ParserError::UnexpectedToken)
        };

        let mut block = Block::new(BlockKind::Input(is_expansive), first_token.level);

        block.tokens = self.read_while(|level, position, _| level == first_token.level && position.0 == (first_token.position.0).0);

        block
    }

    fn parse_output_block(&mut self, first_token: Token) -> Block {
        let mut block = Block::new(BlockKind::Output(first_token.clone()), first_token.level);
        block.tokens = self.read_while(|level, position, _| level == first_token.level && position.0 == (first_token.position.0).0);
        block
    }

    fn parse_rule_declaration_block(&mut self, first_token: Token) -> Block {
        let mut block = Block::new(BlockKind::Undetermined, first_token.level);
        block.tokens = self.read_while(|level, position, _| level == first_token.level && position.0 == (first_token.position.0).0);
        block.kind = BlockKind::RuleDeclaration(block.tokens.remove(0));
        block
    }

    fn parse_rule_invocation_block(&mut self, first_token: Token) -> Block {
        let mut block = Block::new(BlockKind::Undetermined, first_token.level);
        block.tokens = self.read_while(|level, position, _| level == first_token.level && position.0 == (first_token.position.0).0);
        block.kind = BlockKind::RuleInvocation(block.tokens.remove(0));
        block
    }

    fn parse_context_declaration(&mut self, first_token: Token) -> Block {
        let mut block = Block::new(BlockKind::ContextDeclaration, first_token.level);

        // block.children = 

        block
    }

    fn parse_content_block(&mut self, content_token: Token, init_tokens: Vec<Token>) -> Block {
        let mut block = Block::new(BlockKind::Content, content_token.level);

        block
    }

    fn parse_children(&mut self) -> Vec<Block> {
        let mut children = Vec::<Block>::new();



        children
    }
}


// fn parse_block(mut block: Block) -> Block {
//     let mut tokens = block.tokens.into_iter();

    
//     while let Some(token) = tokens.next() {
//         let mut current_block = Block::new(BlockKind::Undetermined, token.level);

//         if token.level == block.level {
//             match token.kind {
//                 
//                 _ => {
//                     panic!(ParserError::UnexpectedToken);
//                 }
//             }
//         }
//         else if token.level > block.level {
//             let first_token_level = token.level;
//             let mut child_block = Block::new(BlockKind::Undetermined, first_token_level);
//             child_block.tokens.push(token);
//             loop {
//                 match tokens.nth(0) {
//                     Some(t) => {
//                         if t.level > first_token_level {
//                             child_block.tokens.push(tokens.next().unwrap());
//                         }
//                         else {
//                             break;
//                         }
//                     }
//                     None => break
//                 }
//             }
//             thread::spawn(move || {
//                 current_block.children.push(parse_block(child_block))
//             });
//         }
//     }

//     block
// }

// fn old_parse_block(mut block: Block) -> Block {
//     let block_data = block.clone();
//     let block_level = block_data.level;
//     let mut block_level_tokens = block_data.tokens.clone().into_iter().filter(|tok| tok.level == block_level);
//     println!("{:?}", block_level_tokens);
//     println!("{:?}", block.tokens);
//     while let Some(start_token) = block_level_tokens.next() {
//         assert_eq!(&start_token, &block.tokens.remove(0));

//         let mut current_block = Block::new(BlockKind::Undetermined, start_token.level);
//         // let mut last_token_index = block_data.tokens.clone().iter().position(|tok| tok == &start_token).unwrap();

//         current_block.tokens.push(start_token.clone());

//         loop {
//             let next = block_level_tokens.nth(0);
//             match next {
//                 Some(next_token) => {
//                     if (next_token.position.0).0 == (start_token.position.0).0 {
//                         current_block.tokens.push(block_level_tokens.next().unwrap());
//                         // last_token_index += 1;
//                         block.tokens.remove(0);
//                     }
//                     else {
//                         break;
//                     }
//                 },
//                 None => break
//             }
//         }

//         if block.tokens.len() != 0 && block.tokens[0].level > current_block.level {
//             let child_block_level = block.tokens[0].level;
//             loop {
//                 let mut child_block = Block::new(BlockKind::Undetermined, child_block_level);
//                 loop {
//                     child_block.tokens.push(block.tokens.remove(0));
//                     if block.tokens[0].level <= current_block.level {
//                         break;
//                     }
//                 }
//                 current_block.children.push(parse_block(child_block));
//                 if block.tokens[0].level < child_block_level {
//                     break;
//                 }
//             }
//         }

//         block.children.push(current_block);
//     }
//     block
// }




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
