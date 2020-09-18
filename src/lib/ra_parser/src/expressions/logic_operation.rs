use super::buffered::{Buffer, Buffered};
use crate::errors::ParserError;
use crate::parsed_by_token::ParsedByToken;
use ra_lexer::token::{RaToken, TokenKind};
use std::rc::Rc;

#[derive(Serialize, Clone, Debug)]
pub enum LogicOperationKind {
    AND,  // &
    OR,   // |
    XOR,  // ||
    NAND, // !&
    NOT,  // !
    NOR,  // !!
    XNOR, // !|
}

#[derive(Serialize, Clone, Debug)]
pub struct LogicOperation {
    kind: Option<LogicOperationKind>,
    #[serde(skip)]
    buffer: Buffer<LogicOperation>,
}

impl<'a> Buffered<'a, LogicOperation> for LogicOperation {
    fn new_candidates_from_token(token: &RaToken<'a>) -> Vec<Rc<LogicOperation>> {
        match token.kind {
            TokenKind::Ampersand => vec![Rc::new(Self {
                kind: Some(LogicOperationKind::AND),
                buffer: Vec::new(),
            })],
            TokenKind::Pipe => vec![
                Rc::new(Self {
                    kind: Some(LogicOperationKind::OR),
                    buffer: Vec::new(),
                }),
                Rc::new(Self {
                    kind: Some(LogicOperationKind::XOR),
                    buffer: Vec::new(),
                }),
            ],
            TokenKind::Exclamation => vec![
                Rc::new(Self {
                    kind: Some(LogicOperationKind::NAND),
                    buffer: Vec::new(),
                }),
                Rc::new(Self {
                    kind: Some(LogicOperationKind::NOT),
                    buffer: Vec::new(),
                }),
                Rc::new(Self {
                    kind: Some(LogicOperationKind::NOR),
                    buffer: Vec::new(),
                }),
                Rc::new(Self {
                    kind: Some(LogicOperationKind::XNOR),
                    buffer: Vec::new(),
                }),
            ],
            _ => vec![],
        }
    }
    fn get_buffer(&self) -> Vec<Rc<LogicOperation>> {
        self.buffer.clone()
    }
}

impl<'a> ParsedByToken<'a, LogicOperation> for LogicOperation {
    fn new(token: RaToken<'a>) -> Result<Box<LogicOperation>, Vec<ParserError>> {
        todo!()
    }
    fn append_token(self, token: RaToken<'a>) -> Result<Box<LogicOperation>, Vec<ParserError>> {
        todo!()
    }
    fn allowed_tokens(&self) -> Vec<TokenKind<'a>> {
        todo!()
    }
    fn starts_with_tokens() -> Vec<TokenKind<'a>> {
        todo!()
    }
}
