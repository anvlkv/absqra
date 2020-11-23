use super::*;
use indextree::{Arena, Node, NodeId};

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
        let paired = token.pair();
        let tree_iter = self.arena.iter();
        let last_added_ref = tree_iter.last().unwrap();
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

        if self.is_current(&token, &last_added) {
            last_child_id.insert_after(inserted_id, &mut self.arena);
        } else {
            let block_id = self.arena.new_node(RaBlock::Block);
            block_id.append(inserted_id, &mut self.arena);

            if self.is_child(&token, &last_added) {
                last_child_id.append(block_id, &mut self.arena);
            } else {
                let parent_block =
                    last_child_id
                        .ancestors(&self.arena)
                        .skip(1)
                        .find(|next_parent_id| {
                            let blk = self.arena.get(*next_parent_id).unwrap();
                            blk.get() == &RaBlock::Block && self.is_sibling(&token, blk)
                        });
                match parent_block {
                    Some(parent_id) => {
                        parent_id.insert_after(block_id, &mut self.arena);
                    }
                    None => {
                        self.root_id.append(block_id, &mut self.arena);
                    }
                }
            }
        }
    }
    fn is_child(&self, token: &RaToken, node: &Node<RaBlock>) -> bool {
        match node.get() {
            RaBlock::Root | RaBlock::Block | RaBlock::Group => match node.last_child() {
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
            RaBlock::Root | RaBlock::Block | RaBlock::Group => match node.last_child() {
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
            RaBlock::Root | RaBlock::Block | RaBlock::Group => match node.last_child() {
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
