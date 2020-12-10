use super::*;
use indextree::{Arena, NodeId, Node};
use serde::{Serialize, Serializer};

#[derive(Serialize, Debug, Clone, PartialEq)]
enum BlockTreeNode {
    Root,
    Block,
    Group,
    Token(RaToken),
}

#[derive(Debug, Clone)]
pub (crate) struct RaTree {
    arena: Arena<BlockTreeNode>,
    awaiting_paired_tokens: Vec<TokenKind>,
    root_id: NodeId,
}


#[derive(Serialize, Debug, Clone)]
pub (crate) enum RaBlock {
    Tokens(Vec<RaToken>),
    Group
}

impl RaTree {
    pub fn traverse<'a>(&'a self) -> impl Iterator<Item = RaBlock> + 'a {
        self.root_id.descendants(&self.arena).filter_map(move |node_id| {
            let node = self.arena.get(node_id).unwrap();
            match node.get() {
                BlockTreeNode::Block
                | BlockTreeNode::Group => {
                    let mut children = node_id.children(&self.arena).peekable();
                    let mut tokens = Vec::new();

                    while let Some(child_id) = children.next() {
                        let child_node = self.arena.get(child_id).unwrap();
                        match child_node.get() {
                            BlockTreeNode::Token(t) => {
                                tokens.push(t.clone());
                                if let Some(next_id) = children.peek() {
                                    let next_node = self.arena.get(*next_id).unwrap();
                                    match next_node.get() {
                                        BlockTreeNode::Token(_) => {},
                                        _ => break
                                    }
                                }
                            },
                            BlockTreeNode::Group => {
                                return Some(RaBlock::Group)
                            }
                            _ => panic!("invalid tree")
                        }
                    }
                    Some(RaBlock::Tokens(tokens))
                }, 
                _ => None
            }
        })
        
        //map(move |d_id| self.arena.get(d_id).unwrap())
    }

    pub(crate) fn new() -> Self {
        let mut arena = Arena::new();
        let root_id = arena.new_node(BlockTreeNode::Root);
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
                BlockTreeNode::Root => {
                    let block_id = self.arena.new_node(BlockTreeNode::Block);
                    let inserted_id = self.append_token(token);
                    block_id.append(inserted_id, &mut self.arena);
                    node_id.append(block_id, &mut self.arena);
                    break;
                }
                BlockTreeNode::Group => {
                    let group_is_open = {
                        let opening_node_id = node.first_child().unwrap();
                        let closing_node_id = node.last_child().unwrap();
                        opening_node_id == closing_node_id || {
                            let opening_node = self.arena.get(opening_node_id).unwrap();
                            let closing_node = self.arena.get(closing_node_id).unwrap();
                            match (opening_node.get(), closing_node.get()) {
                                (BlockTreeNode::Token(t1), BlockTreeNode::Token(t2)) => {
                                    t1.closing_pair().unwrap() != t2.kind
                                }
                                _ => true,
                            }
                        }
                    };

                    if group_is_open {
                        let mut inserted_id = self.append_token(token);
                        if is_child_to_previous_token {
                            let block_id = self.arena.new_node(BlockTreeNode::Block);
                            block_id.append(inserted_id, &mut self.arena);
                            inserted_id = block_id
                        }

                        node_id.append(inserted_id, &mut self.arena);
                        break;
                    }
                }
                BlockTreeNode::Block => {
                    if is_child_to_previous_token {
                        let inserted_id = self.append_token(token);
                        let block_id = self.arena.new_node(BlockTreeNode::Block);
                        block_id.append(inserted_id, &mut self.arena);
                        node_id.append(block_id, &mut self.arena);
                        break;
                    } else if is_sibling_to_previous_token {
                        let inserted_id = self.append_token(token);
                        node_id.append(inserted_id, &mut self.arena);
                        break;
                    }
                }
                BlockTreeNode::Token(compare_token) => {
                    is_child_to_previous_token = compare_token.level == token.level - 1;
                    is_sibling_to_previous_token =
                        (compare_token.position.0).line() == (token.position.0).line();
                }
            }
        }
    }

    fn append_token(&mut self, token: RaToken) -> NodeId {
        let mut inserted_id = self.arena.new_node(BlockTreeNode::Token(token.clone()));
        if let Some(pair) = token.closing_pair() {
            self.awaiting_paired_tokens.push(pair);
            let group_id = self.arena.new_node(BlockTreeNode::Group);
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
