use super::*;
use indextree::{Arena, NodeId};
use std::convert::{TryFrom};
use ra_tree_parser::serialize_tree::SerializeTree;
use serde::ser::{Serialize, Serializer};

pub struct RaAST {
    arena: Arena<RaASTNode>,
    root_id: NodeId
}

impl TryFrom<RaTree> for RaAST {
    type Error = (Option<RaAST>, Vec<ParserError>);
    fn try_from(tree: RaTree) -> Result<RaAST, Self::Error> {
        let mut arena = Arena::new();
        let mut traverse_iter = tree.traverse();
        assert!(traverse_iter.next().unwrap().get() == &RaBlock::Root);
        let root_id = arena.new_node(RaASTNode::Root);

        while let Some(block_node) = traverse_iter.next() {
            match block_node.get() {
                RaBlock::Block => {

                },
                RaBlock::Group => {
                    
                },
                RaBlock::Token(_) => {

                }
                _ => panic!("invalid tree")
            }
        }

        Ok(Self {
            arena,
            root_id
        })
    }
}

impl Serialize for RaAST {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let ser_node = SerializeTree::new(self.root_id, &self.arena);
        ser_node.serialize(serializer)
    }
}