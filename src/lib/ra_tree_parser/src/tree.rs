use super::*;
use serde_indextree::{SiblingNodes};
use indextree::{Arena, NodeEdge, Node};
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum RaBlock {
    Block,
    Group,
    Token(RaToken),
}

pub struct RaTree {
    arena: Arena<RaBlock>,
    awaiting_paired_tokens: Vec<TokenKind>,
}

impl RaTree {
    pub(crate) fn new() -> Self {
        Self {
            arena: Arena::new(),
            awaiting_paired_tokens: Vec::new(),
        }
    }

    pub(crate) fn add_token(&mut self, token: RaToken) {
        let paired = token.pair();
        let tree_iter = self.arena.iter();
        match tree_iter.last() {
            Some(last_added_ref) => {
                let last_added = last_added_ref.clone();
                let last_child_id = last_added
                    .last_child()
                    .unwrap_or(self.arena.get_node_id(last_added_ref).unwrap());

                let mut inserted_id = self.arena.new_node(RaBlock::Token(token.clone()));

                if let Some(paired) = paired {
                    self.awaiting_paired_tokens.push(paired);
                    let group_id = self.arena.new_node(RaBlock::Group);
                    group_id.append(inserted_id, &mut self.arena);
                    inserted_id = group_id;
                }

                match token {
                    t if self.is_current(&t, &last_added) => {
                        last_child_id.insert_after(inserted_id, &mut self.arena);
                    }
                    t if self.is_child(&t, &last_added) => {
                        let block_id = self.arena.new_node(RaBlock::Block);
                        block_id.append(inserted_id, &mut self.arena);
                        last_child_id.append(block_id, &mut self.arena);
                    }
                    t if self.is_sibling(&t, &last_added) => {
                        let mut parents_iter = last_child_id.reverse_traverse(&self.arena);
                        while let Some(edge) = parents_iter.next() {
                            match edge {
                                NodeEdge::Start(next_parent_id) => {
                                    match self.arena.get(next_parent_id).unwrap().get() {
                                        RaBlock::Block => {
                                            next_parent_id
                                                .insert_after(inserted_id, &mut self.arena);
                                            break;
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
            None => {
                let mut inserted_id = self.arena.new_node(RaBlock::Token(token.clone()));

                if let Some(paired) = paired {
                    self.awaiting_paired_tokens.push(paired);
                    let group_id = self.arena.new_node(RaBlock::Group);
                    group_id.append(inserted_id, &mut self.arena);
                    inserted_id = group_id;
                }

                let block_id = self.arena.new_node(RaBlock::Block);

                block_id.append(inserted_id, &mut self.arena);
            }
        }
    }
    
    fn is_child(&self, token: &RaToken, node: &Node<RaBlock>) -> bool {
        match node.get() {
            RaBlock::Block | RaBlock::Group => match node.last_child() {
                Some(child_id) => self.is_child(token, self.arena.get(child_id).unwrap()),
                None => false,
            },
            RaBlock::Token(last_token) => {
                token.level > last_token.level && token.position > last_token.position
            }
        }
    }
    
    fn is_sibling(&self, token: &RaToken, node: &Node<RaBlock>) -> bool {
        match node.get() {
            RaBlock::Block | RaBlock::Group => match node.last_child() {
                Some(child_id) => self.is_sibling(token, self.arena.get(child_id).unwrap()),
                None => false,
            },
            RaBlock::Token(last_token) => {
                token.level == last_token.level && !self.is_current(token, node)
            }
        }
    }
    
    fn is_current(&self, token: &RaToken, node: &Node<RaBlock>) -> bool {
        match node.get() {
            RaBlock::Block | RaBlock::Group => match node.last_child() {
                Some(child_id) => self.is_current(token, self.arena.get(child_id).unwrap()),
                None => false,
            },
            RaBlock::Token(last_token) => {
                token.level == last_token.level
                    && (token.position.0).line() == (last_token.position.0).line()
            }
        }
    }
}

impl Serialize for RaTree {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where S: Serializer { 
        let first_node = self.arena.iter().nth(0);
        match first_node {
            Some(first_node) => {
                let serialize_node = SiblingNodes::new(self.arena.get_node_id(first_node).unwrap(), &self.arena);
                serialize_node.serialize(serializer)
            },
            None => {
                let s = serializer.serialize_struct("", 0)?;
                s.end()
            }
        }
    }
}
