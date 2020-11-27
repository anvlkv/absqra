use super::*;
use indextree::{Arena, Node, NodeId};
use serde::ser::SerializeSeq;
use serde::{Serialize, Serializer};

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum RaBlock {
    Root,
    Block,
    Group,
    Token(RaToken),
}

#[derive(Debug, Clone)]
pub struct RaTree {
    arena: Arena<RaBlock>,
    awaiting_paired_tokens: Vec<TokenKind>,
    root_id: NodeId,
}

impl RaTree {
    pub(crate) fn new() -> Self {
        let mut arena = Arena::new();
        let root_id = arena.new_node(RaBlock::Root);
        Self {
            arena,
            root_id,
            awaiting_paired_tokens: Vec::new(),
        }
    }

    pub(crate) fn add_token(&mut self, token: RaToken) {
        
    }
}

impl Serialize for RaTree {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let ser_node = SerializeTree::new(self.root_id, &self.arena);
        ser_node.serialize(serializer)
    }
}
