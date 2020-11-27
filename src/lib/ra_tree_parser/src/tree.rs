use super::*;
use indextree::{Arena, NodeId};
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
        let mut iter_up = self
            .root_id
            .descendants(&self.arena)
            .last()
            .unwrap()
            .ancestors(&self.arena);
        let mut is_sibling_to_previous_token = false;
        let mut is_child_to_previous_token = false;

        while let Some(node_id) = iter_up.next() {
            let node = self.arena.get(node_id).unwrap();
            match node.get() {
                RaBlock::Root => {
                    let block_id = self.arena.new_node(RaBlock::Block);
                    let inserted_id = self.append_token(token);
                    block_id.append(inserted_id, &mut self.arena);
                    node_id.append(block_id, &mut self.arena);
                    break;
                }
                RaBlock::Group => {
                    let group_is_open = {
                        let opening_node_id = node.first_child().unwrap();
                        let closing_node_id = node.last_child().unwrap();
                        opening_node_id == closing_node_id || {
                            let opening_node = self.arena.get(opening_node_id).unwrap();
                            let closing_node = self.arena.get(closing_node_id).unwrap();
                            match (opening_node.get(), closing_node.get()) {
                                (RaBlock::Token(t1), RaBlock::Token(t2)) => {
                                    t1.closing_pair().unwrap() != t2.kind
                                }
                                _ => true,
                            }
                        }
                    };

                    if group_is_open {
                        let mut inserted_id = self.append_token(token);
                        if is_child_to_previous_token {
                            let block_id = self.arena.new_node(RaBlock::Block);
                            block_id.append(inserted_id, &mut self.arena);
                            inserted_id = block_id
                        }

                        node_id.append(inserted_id, &mut self.arena);
                        break;
                    }
                }
                RaBlock::Block => {
                    if is_child_to_previous_token {
                        let inserted_id = self.append_token(token);
                        let block_id = self.arena.new_node(RaBlock::Block);
                        block_id.append(inserted_id, &mut self.arena);
                        node_id.append(block_id, &mut self.arena);
                        break;
                    } else if is_sibling_to_previous_token {
                        let inserted_id = self.append_token(token);
                        node_id.append(inserted_id, &mut self.arena);
                        break;
                    }
                }
                RaBlock::Token(compare_token) => {
                    is_child_to_previous_token = compare_token.level == token.level - 1;
                    is_sibling_to_previous_token =
                        (compare_token.position.0).line() == (token.position.0).line();
                }
            }
        }
    }

    fn append_token(&mut self, token: RaToken) -> NodeId {
        let mut inserted_id = self.arena.new_node(RaBlock::Token(token.clone()));
        if let Some(pair) = token.closing_pair() {
            self.awaiting_paired_tokens.push(pair);
            let group_id = self.arena.new_node(RaBlock::Group);
            group_id.append(inserted_id, &mut self.arena);
            inserted_id = group_id;
        } else if let Some(index) = self
            .awaiting_paired_tokens
            .iter()
            .position(|k| k == &token.kind)
        {
            self.awaiting_paired_tokens.remove(index);
        }
        inserted_id
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
