use ra_lexer::token::{Token, TokenKind};
use ra_lexer::cursor::Position;


pub(crate) struct Cursor<'a> {
    pub position: Position,
    pub level: usize,
    tokens: Box<dyn Iterator<Item = Token> + 'a>,
    tokens_vec: Vec<Token>,
    consumed_len: usize
}

impl <'a> Cursor<'a> {
    pub fn new(tokens: Box<dyn Iterator<Item = Token> + 'a>) -> Cursor<'a> {
        let tokens_vec: Vec<Token> = tokens.collect();
        let first_token = &tokens_vec[0];
        
        Cursor {
            position: first_token.position.0,
            level: first_token.level,
            tokens: Box::new(tokens_vec.clone().into_iter()),
            consumed_len: 0,
            tokens_vec
        }
    }

    fn nth(&mut self, n: usize) -> Option<Token> {
        if self.tokens_vec.len() > self.consumed_len + n + 1 {
            Some(self.tokens_vec[self.consumed_len + n].clone())
        }
        else {
            None
        }
    }

    pub fn first_ahead(&mut self) -> Option<Token> {
        self.nth(0)
    }

    pub fn second_ahead(&mut self) -> Option<Token> {
        self.nth(1)
    }

    pub fn bump(&mut self) -> Option<Token> {
        let tok = self.tokens.next();
        match tok {
            Some(token) => {
                self.position = token.position.1;
                self.level = token.level;
                self.consumed_len += 1;
                return Some(token);
            }
            None => None
        }
    }

    pub fn read_while<F>(&mut self, mut test: F) -> Vec<Token> 
        where F: FnMut (usize, Position, TokenKind) -> bool 
    {
        let mut read: Vec<Token> = Vec::new();

        loop {
            let next_token = self.first_ahead();
            match next_token {
                Some(t) => {
                    if test(t.level, t.position.0, t.kind) {
                        read.push(self.bump().unwrap())
                    }
                    else {
                        break
                    }
                },
                None => break
            }
        }

        read
    }

    pub fn read_within(&mut self, open: TokenKind, close: TokenKind) -> Vec<Token> {
        let mut read: Vec<Token> = Vec::new();

        loop {
            let next_token = self.first_ahead();

            match next_token {
                Some(tok) => {
                    match tok.kind {
                        k if k == open => {
                            let open_token = self.bump().unwrap();
                            let mut nested = self.read_within(open_token.kind, close.clone());
                            read.append(&mut nested);
                        },
                        k if k == close => {
                            self.bump();
                            break
                        },
                        _ => read.push(self.bump().unwrap())
                    }
                },
                None => break
            }

            // println!("{:?}", &read);
        }

        read
    }

    pub fn is_eof(&mut self) -> bool {
        self.consumed_len == self.tokens_vec.len()
    }
}