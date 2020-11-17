use super::*;
use ra_lexer::TokenKind;

#[derive(Serialize, Debug, Clone)]
pub enum TreeNodeMember {
    Token(RaToken),
    SubTree(Box<RaTree>),
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct RaTree {
    pub node: Vec<TreeNodeMember>,
    pub level: u16,
    pub position: (Position, Position),
    pub children: Vec<Box<RaTree>>,
    #[serde(skip_serializing)]
    awaited_paired_tokens: Vec<TokenKind>,
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

    fn get_pair(token: &RaToken) -> Option<TokenKind> {
        match token.kind {
            TokenKind::OpenCurlyBrace => Some(TokenKind::CloseCurlyBrace),
            TokenKind::OpenSquareBrace => Some(TokenKind::CloseSquareBrace),
            TokenKind::OpenParentheses => Some(TokenKind::CloseParentheses),
            _ => None,
        }
    }

    pub(crate) fn push_token(&mut self, token: RaToken) -> Result<(), Vec<TreeParserError>> {
        let (Position(start_line, start_col), _) = self.position;
        let (_, Position(token_end_line, token_end_col)) = token.position;

        self.position = (
            Position(start_line, start_col),
            Position(token_end_line, token_end_col),
        );
        if self.awaited_paired_tokens.len() == 0 {
            match &token {
                t if self.is_primary(t) => match Self::get_pair(t) {
                    Some(p) => {
                        self.awaited_paired_tokens.push(p);
                        self.node
                            .push(TreeNodeMember::SubTree(Box::new(RaTree::from(token))));
                        Ok(())
                    }
                    None => {
                        self.node.push(TreeNodeMember::Token(token));
                        Ok(())
                    }
                },
                t if self.children.is_empty() || self.children.last().unwrap().is_sibling(t) => {
                    self.children.push(Box::new(RaTree::from(token)));
                    Ok(())
                }
                t if self.is_child(t) => {
                    let last_child_index = self.children.len() - 1;
                    self.children[last_child_index].push_token(token)
                }
                _ => Err(vec![TreeParserError::InvalidTree(
                    token.position.0,
                    Backtrace::new(),
                )]),
            }
        } else {
            match self
                .awaited_paired_tokens
                .iter()
                .position(|kind| kind == &token.kind)
            {
                Some(p) => {
                    self.awaited_paired_tokens.remove(p);
                }
                _ => {}
            };

            match self.node.pop().unwrap() {
                TreeNodeMember::SubTree(mut sub_tree) => {
                    sub_tree.push_token(token)?;
                    self.node.push(TreeNodeMember::SubTree(sub_tree));
                    Ok(())
                }
                _ => panic!("unexpected TreeNodeMember")
            }
        }
    }

    pub fn no_comments(self) -> RaTree {
        Self {
            node: self
                .node
                .into_iter()
                .filter(|t| match t {
                    TreeNodeMember::SubTree(_) => true,
                    TreeNodeMember::Token(t) => {
                        t.kind != TokenKind::Comment(String::default(), true)
                            && t.kind != TokenKind::Comment(String::default(), false)
                    }
                })
                .map(|t| match t {
                    TreeNodeMember::SubTree(sub_tree) => TreeNodeMember::SubTree(Box::new(sub_tree.no_comments())),
                    TreeNodeMember::Token(_) => t
                })
                .collect(),
            children: self
                .children
                .into_iter()
                .map(|c| Box::new(c.no_comments()))
                .collect(),
            ..self
        }
    }
}

impl<'a> From<RaToken> for RaTree {
    fn from(token: RaToken) -> Self {
        let level = token.level;
        let position = token.position;
        let mut awaited_paired_tokens = Vec::new();
        let mut node = Vec::new();

        match Self::get_pair(&token) { 
            Some(p) => {
                awaited_paired_tokens.push(p);
                node.push(TreeNodeMember::SubTree(Box::new(RaTree {
                    node: vec![TreeNodeMember::Token(token)],
                    level,
                    position,
                    children: Vec::new(),
                    awaited_paired_tokens: Vec::new()
                })))
            }
            None => {
                node.push(TreeNodeMember::Token(token))
            }
        }

        Self {
            node,
            level,
            position,
            children: Vec::new(),
            awaited_paired_tokens,
        }
    }
}
