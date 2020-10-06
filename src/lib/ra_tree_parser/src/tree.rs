use ra_lexer::cursor::Position;
use serde::Serialize;
use super::*;


#[derive(Serialize, Debug, Clone, Default)]
pub struct RaTree<'a> {
    tokens: Vec<RaToken<'a>>,
    level: u16,
    position: (Position, Position),
    children: Vec<Box<RaTree<'a>>>
}

impl <'a> RaTree<'a> {
    fn is_primary(&self, token: &RaToken<'a>) -> bool {
        let (Position(start_line, _), _) = self.position;
        let (Position(token_start_line, _), _) = token.position;
        let level = self.level;

        start_line == token_start_line && level == token.level
    }

    fn is_sibling(&self, token: &RaToken<'a>) -> bool {
        let level = self.level;
        level == token.level && !self.is_primary(&token)
    }

    fn is_child(&self, token: &RaToken<'a>) -> bool {
        let level = self.level;
        let (Position(start_line, _), _) = self.position;
        let (Position(token_start_line, _), _) = token.position;

        token.level > level && token_start_line > start_line
    }

    pub fn push_token(self, token: RaToken<'a>) -> Result<RaTree<'a>, Vec<TreeParserError>> {
        let (Position(start_line, start_col), _) = self.position;
        let (_, Position(token_end_line, token_end_col)) = token.position;
        let position = (Position(start_line, start_col), Position(token_end_line, token_end_col));

        if self.is_primary(&token) {
            let mut tokens = self.tokens;
            tokens.push(token);
            Ok(Self{
                tokens,
                position,
                ..self
            })
        }
        else {
            let mut children = self.children.clone();
            if children.is_empty() || children.last().unwrap().is_sibling(&token) {
                children.push(Box::new(RaTree::from(token)));
            }
            else if self.is_child(&token) {
                let last_child = children.pop().unwrap();
                children.push(Box::new(last_child.push_token(token)?));
            }
            else {
                return Err(vec![TreeParserError::Error]);
            }

            Ok(Self {
                children,
                position,
                ..self
            })
        }
    }
}

impl <'a> From<RaToken<'a>> for RaTree<'a> {
    fn from(token: RaToken<'a>) -> Self { 
        Self {
            tokens: vec![token],
            level: token.level,
            position: token.position,
            children: Vec::new()
        }
    }
}