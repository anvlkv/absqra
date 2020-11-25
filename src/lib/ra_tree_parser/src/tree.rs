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
        let paired = token.pair();
        let last_added_ref = self.arena.iter().last().unwrap();
        let last_added = last_added_ref.clone();
        let last_child_id = self
            .arena
            .get_node_id(last_added_ref)
            .unwrap()
            .descendants(&self.arena)
            .last()
            .unwrap_or(self.arena.get_node_id(&last_added).unwrap());

        let mut inserted_id = self.arena.new_node(RaBlock::Token(token.clone()));

        if let Some(paired) = paired {
            let group_id = self.arena.new_node(RaBlock::Group);
            self.awaiting_paired_tokens.push(paired);
            group_id.append(inserted_id, &mut self.arena);
            inserted_id = group_id;
        } else if let Some(index) = self
            .awaiting_paired_tokens
            .iter()
            .position(|pair| pair == &token.kind)
        {
            self.awaiting_paired_tokens.remove(index);
        }

        let mut iter_up = last_child_id.ancestors(&self.arena);

        let mut added = false;

        while let Some(parent_id) = iter_up.next() {
            let parent = self.arena.get(parent_id).unwrap().clone();
            if self.is_child(&token, &parent) {
                parent_id.append(inserted_id, &mut self.arena);
                added = true;
                break;
            }
            else if parent_id == self.root_id || self.is_sibling(&token, &parent) {
                let block_id = self.arena.new_node(RaBlock::Block);
                block_id.append(inserted_id, &mut self.arena);
                if self.is_sibling(&token, &parent) {
                    parent_id.insert_after(block_id, &mut self.arena);
                } else {
                    parent_id.append(block_id, &mut self.arena);
                }
                added = true;
                break;
            }
        }

        if !added {
            panic!("invalid tree")
        }
    }
    fn is_child(&self, token: &RaToken, node: &Node<RaBlock>) -> bool {
        let compare_node = node.get();
        let node_id = self.arena.get_node_id(node).unwrap();
        let mut node_descendants = node_id.descendants(&self.arena).skip(1);

        match compare_node {
            RaBlock::Root => false,
            RaBlock::Token(_) => false,
            RaBlock::Block => {
                while let Some(child_node_id) = node_descendants.next() {
                    match self.arena.get(child_node_id).unwrap().get() {
                        RaBlock::Token(_) => {
                            return self.is_sibling(token, self.arena.get(child_node_id).unwrap())
                        }
                        _ => {}
                    }
                }
                false
            }
            RaBlock::Group => {
                let opening_id = node_id.children(&self.arena).nth(0).unwrap();
                let closing_id = node_id.children(&self.arena).last().unwrap();

                let opening_node_data = self.arena.get(opening_id).unwrap().get();
                let closing_node_data = self.arena.get(closing_id).unwrap().get();

                let group_closed = match (opening_node_data, closing_node_data) {
                    (RaBlock::Token(t1), RaBlock::Token(t2)) => t1.pair().unwrap() == t2.kind,
                    _ => false,
                };

                !group_closed && {
                    while let Some(child_node_id) = node_descendants.next() {
                        match self.arena.get(child_node_id).unwrap().get() {
                            RaBlock::Token(_) => {
                                return self
                                    .is_sibling(token, self.arena.get(child_node_id).unwrap())
                            }
                            _ => {}
                        }
                    }
                    false
                }
            }
        }
    }

    fn is_sibling(&self, token: &RaToken, node: &Node<RaBlock>) -> bool {
        let compare_node = node.get();
        let node_id = self.arena.get_node_id(node).unwrap();
        // let mut node_descendants = node_id.descendants(&self.arena).skip(1);

        match compare_node {
            RaBlock::Root => false,
            RaBlock::Token(compare_token) => {
                compare_token.level == token.level
                    && (compare_token.position.0).line() == (token.position.0).line()
            }
            RaBlock::Block => {
                while let Some(child_node_id) = node_descendants.next() {
                    match self.arena.get(child_node_id).unwrap().get() {
                        RaBlock::Token(compare_token) => {
                            return compare_token.level == token.level;
                        }
                        _ => {}
                    }
                }
                false
            },
            RaBlock::Group => {
                let opening_id = node_id.children(&self.arena).nth(0).unwrap();
                let closing_id = node_id.children(&self.arena).last().unwrap();

                let opening_node = self.arena.get(opening_id).unwrap();
                let closing_node = self.arena.get(closing_id).unwrap();

                match (opening_node.get(), closing_node.get()) {
                    (RaBlock::Token(t1), RaBlock::Token(t2)) => {
                        let group_closed = t1.pair().unwrap() == t2.kind;
                        group_closed && self.is_sibling(token, closing_node)
                    }
                    _ => false,
                }
            }
        }
        // match node.get() {
        //     RaBlock::Root | RaBlock::Block => match node.last_child() {
        //         Some(child_id) => self.is_sibling(token, self.arena.get(child_id).unwrap()),
        //         None => false,
        //     },
        //     RaBlock::Group => match node.last_child() {
        //         Some(child_id) => {
        //             let opening_block = {
        //                 let first_child_id = node.first_child().unwrap();
        //                 let token_id = first_child_id.descendants(&self.arena).last().unwrap();
        //                 self.arena.get(token_id).unwrap().get()
        //             };
        //             let closing_block = {
        //                 let token_id = child_id.descendants(&self.arena).last().unwrap();
        //                 self.arena.get(token_id).unwrap().get()
        //             };

        //             match (opening_block, closing_block) {
        //                 (RaBlock::Token(t1), RaBlock::Token(t2)) => {
        //                     println!("{:?}", t1);
        //                     println!("{:?}", t2);
        //                     let pair = t1.pair().unwrap();
        //                     pair != t2.kind && self.is_sibling(token, self.arena.get(child_id).unwrap())
        //                 },
        //                 _ => panic!("invalid group")
        //             }
        //         },
        //         None => false,
        //     },
        //     RaBlock::Token(last_token) => {
        //         token.level == last_token.level && !self.is_current(token, node)
        //     }
        // }
    }
}

impl Serialize for RaTree {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let root = self.arena.iter().nth(0);

        match root {
            Some(r) => {
                let ser_node = SerializeTree::new(self.arena.get_node_id(r).unwrap(), &self.arena);
                ser_node.serialize(serializer)
            }
            None => {
                let s = serializer.serialize_seq(None)?;
                s.end()
            }
        }
    }
}
