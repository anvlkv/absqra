use super::*;
use ra_lexer::TokenKind;

#[derive(Serialize, Debug, Clone, Default)]
pub struct RaTree {
    pub tokens: Vec<RaToken>,
    pub level: u16,
    pub position: (Position, Position),
    pub children: Vec<Box<RaTree>>
}

impl RaTree {
    fn is_primary(&self, token: &RaToken) -> bool {
        let (Position(start_line, _), _) = self.position;
        let (Position(token_start_line, _), _) = token.position;
        let level = self.level;

        start_line == token_start_line && level == token.level
    }

    fn is_sibling(&self, token: &RaToken) -> bool {
        let level = self.level;
        level == token.level && !self.is_primary(&token)
    }

    fn is_child(&self, token: &RaToken) -> bool {
        let level = self.level;
        let (Position(start_line, _), _) = self.position;
        let (Position(token_start_line, _), _) = token.position;

        token.level > level && token_start_line > start_line
    }

    pub (crate) fn push_token(&mut self, token: RaToken) -> Result<(), Vec<TreeParserError>> {
        let (Position(start_line, start_col), _) = self.position;
        let (_, Position(token_end_line, token_end_col)) = token.position;
        
        self.position = (Position(start_line, start_col), Position(token_end_line, token_end_col));

        if self.is_primary(&token) {
            self.tokens.push(token);
            Ok(())
        }
        else {
            if self.children.is_empty() || self.children.last().unwrap().is_sibling(&token) {
                self.children.push(Box::new(RaTree::from(token)));
                Ok(())
            }
            else if self.is_child(&token) {
                let last_child_index = self.children.len() - 1;
                self.children[last_child_index].push_token(token)
            }
            else {
                Err(vec![TreeParserError::InvalidTree(token.position.0, Backtrace::new())])
            }
        }
    }

    pub fn no_comments(self) -> RaTree {
        
        Self {
            tokens: self.tokens
            .into_iter()
            .filter(|t| 
                t.kind != TokenKind::Comment(String::default(), true) 
                && t.kind != TokenKind::Comment(String::default(), false)
            )
            .collect(),
            children: self.children.into_iter().map(|c| Box::new(c.no_comments())).collect(),
            level: self.level,
            position: self.position
        }
    }
}

impl <'a> From<RaToken> for RaTree {
    fn from(token: RaToken) -> Self { 
        Self {
            tokens: vec![token.clone()],
            level: token.level,
            position: token.position,
            children: Vec::new()
        }
    }
}