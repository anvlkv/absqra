use super::*;
use indextree::{Arena, NodeId};
use std::convert::{TryFrom};
use block_tree::serialize_tree::SerializeTree;
use serde::ser::{Serialize, Serializer};

#[derive(Debug)]
pub struct RaAST {
    arena: Arena<RaASTNode>,
    root_id: NodeId
}

impl TryFrom<RaTree> for RaAST {
    type Error = (Option<RaAST>, Vec<ParserError>);
    fn try_from(tree: RaTree) -> Result<RaAST, Self::Error> {
        let mut errors = vec![];
        let mut arena = Arena::new();
        let mut traverse_iter = tree.traverse();
        let root_id = arena.new_node(RaASTNode::Root);

        while let Some(block_node) = traverse_iter.next() {
            match RaASTNode::try_from(block_node) {
                Ok(data) => {
                    let id = arena.new_node(data);
                },
                Err((parsed, err)) => {
                    errors.extend(err.into_iter());
                    match parsed {
                        Some(data) => {
                            let id = arena.new_node(data);
                        }
                        None => {}
                    }
                }
            }
        }

        let result = Self {
            arena,
            root_id
        };

        if errors.len() == 0 {
            Ok(result)
        }
        else {
            Err((Some(result), errors))
        }
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