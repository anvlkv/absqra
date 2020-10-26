use super::*;
use std::hash::Hash;

#[derive(Serialize, Debug, Clone)]
pub struct Identifier {
    pub token: RaToken
}

impl Hash for Identifier {
    fn hash<H>(&self, hasher: &mut H) where H: std::hash::Hasher { 
        match &self.token.kind {
            TokenKind::Identifier(id) => {
                String::hash(&id, hasher);
            },
            _ => panic!("expected RaToken kind Identifier")
        }
    }
}

impl Identifier {
    pub (crate) fn new(token: RaToken)-> Self {
        assert!(token.kind == TokenKind::Identifier(String::default()));
        Self {
            token
        }
    }
}