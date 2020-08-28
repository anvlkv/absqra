use ra_lexer::token::{Token};
use ra_lexer::cursor::Position;
use core::iter::Peekable;

pub(crate) struct Cursor<'token, I> 
    where I: Iterator<Item = Token<'token>> 
{
    pub position: Position,
    pub level: u16,
    tokens: Peekable<I>,
    consumed_len: usize
}

impl <'token, I> Cursor<'token, I> 
where I: Iterator<Item = Token<'token>> 
{
    pub fn new(tokens: I) -> Self
        
    {
        Self {
            position: Position(0, 0),
            level: 0,
            tokens: tokens.peekable(),
            consumed_len: 0
        }
    }

    pub fn first_ahead(&mut self) -> Option<&Token<'token>> {
        self.tokens.peek()
    }

    pub fn bump(&mut self) -> Option<Token<'token>> {
        let tok = self.tokens.next();
        match tok {
            Some(token) => {
                self.position = token.position.1;
                self.level = token.level;
                self.consumed_len += 1;
                Some(token)
            }
            None => None
        }
    }

    pub fn is_eof(& mut self) -> bool {
        self.first_ahead() == None
    }

    // pub fn do_while<F, D>(&mut self, mut test: F, mut cb: D)
    //     where F: FnMut (u16, Position, TokenKind) -> bool,
    //     D: FnMut (Token)
    // {
    //     loop {
    //         let next_token = self.first_ahead();
    //         match next_token {
    //             Some(t) => {
    //                 if test(t.level, t.position.0, t.kind) {
    //                     cb(self.bump().unwrap())
    //                 }
    //                 else {
    //                     break
    //                 }
    //             },
    //             None => break
    //         }
    //     }
    // }

    // pub fn read_within(&mut self, open: TokenKind, close: TokenKind) -> Vec<Token> {
    //     let mut read: Vec<Token> = Vec::new();

    //     loop {
    //         let next_token = self.first_ahead();

    //         match next_token {
    //             Some(tok) => {
    //                 match tok.kind {
    //                     k if k == open => {
    //                         let open_token = self.bump().unwrap();
    //                         let mut nested = self.read_within(open_token.kind, close.clone());
    //                         read.append(&mut nested);
    //                     },
    //                     k if k == close => {
    //                         self.bump();
    //                         break
    //                     },
    //                     _ => read.push(self.bump().unwrap())
    //                 }
    //             },
    //             None => break
    //         }

    //         // println!("{:?}", &read);
    //     }

    //     read
    // }
}