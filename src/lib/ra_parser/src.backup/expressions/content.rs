use ra_lexer::cursor::{Cursor, Position};
use ra_lexer::token::{RaToken, TokenKind};
use ra_lexer::tokenize;
use serde::Serialize;

use super::reference_expression::ReferenceExpression;
use super::traits::Expandable;
use failure::Backtrace;
use super::errors::ParserError;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum ContentBlockMember<'a> {
    Body(String),
    Template(ReferenceExpression<'a>),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Content<'a>(ContentBlockMember<'a>, Option<Box<Content<'a>>>);

impl<'a> Content<'a> {
    pub fn new(token: RaToken<'a>) -> Result<Content<'a>, ParserError> {
        match token.kind.unwrap() {
            TokenKind::ContentBlock => Self::parse_content(token),
            _ => Err(ParserError::ExpectedAGotB(
                format!("{}", token),
                format!("{:?}", vec![TokenKind::ContentBlock]),
                token.position.0,
                Backtrace::new()
            )),
        }
    }

    fn parse_content(token: RaToken<'a>) -> Result<Content<'a>, ParserError> {
        let RaToken {
            kind,
            level,
            len,
            content,
            position,
        } = token;

        let mut cursor = Cursor::new(content, position.0, level, 0);

        let mut content = Content(Self::parse_member(&mut cursor)?, None);

        while !cursor.is_eof() {
            match Self::parse_member(&mut cursor) {
                Ok(member) => {
                    content = content.append_item(member)?;
                }
                Err(e) => return Err(e),
            }
        }

        Ok(content)
    }

    fn parse_member(cursor: &mut Cursor<'a>) -> Result<ContentBlockMember<'a>, ParserError> {
        let mut string_buffer = String::new();
        let mut token_buffer = cursor.slice(0, 0);

        while let Some(ch) = cursor.bump() {
            match ch {
                '{' => {
                    match cursor.first_ahead() {
                        '{' => {
                            if string_buffer.len() > 0 {
                                return Ok(ContentBlockMember::Body(string_buffer));
                            }
                            let start_consumed = cursor.len_consumed();
                            while let Some(ch) = cursor.bump() {
                                match ch {
                                    '}' => match cursor.first_ahead() {
                                        '}' => {
                                            if token_buffer.len() > 0 {
                                                return Ok(ContentBlockMember::Template(
                                                    Self::parse_reference_from_buffer(
                                                        token_buffer,
                                                        cursor.position,
                                                    )?,
                                                ));
                                            }
                                            break;
                                        }
                                        _ => return Err(ParserError::InvalidBlock(Backtrace::new())),
                                    },
                                    _ => {
                                        token_buffer =
                                            cursor.slice(start_consumed, cursor.len_consumed())
                                        // token_buffer.push(ch)
                                    }
                                }
                            }
                        }
                        '\\' => {
                            cursor.bump();
                        }
                        _ => {
                            string_buffer.push(ch);
                        }
                    }
                }
                _ => {
                    string_buffer.push(ch);
                }
            }
        }

        if token_buffer.len() > 0 {
            Err(ParserError::UnexpectedEndOfInput(cursor.position, Backtrace::new()))
        } else {
            Ok(ContentBlockMember::Body(string_buffer))
        }
    }

    fn parse_reference_from_buffer(
        buffer: &'a str,
        position: Position,
    ) -> Result<ReferenceExpression<'a>, ParserError> {
        let mut tokens_stream = tokenize(buffer);

        match tokens_stream.next() {
            Some(token) => {
                let mut expression = ReferenceExpression::new(token?)?;

                while let Some(token) = tokens_stream.next() {
                    expression = expression.append_item(token?)?;
                }

                Ok(expression)
            }
            None => Err(ParserError::UnexpectedEndOfInput(position, Backtrace::new())),
        }
    }
}

impl<'a> Expandable<'a, Content<'a>, ContentBlockMember<'a>> for Content<'a> {
    fn append_item(self, item: ContentBlockMember<'a>) -> Result<Content<'a>, ParserError> {
        let Content(current, next) = self;
        match next {
            Some(next) => Ok(Content(current, Some(Box::new(next.append_item(item)?)))),
            None => Ok(Content(current, Some(Box::new(Content(item, None))))),
        }
    }
}
