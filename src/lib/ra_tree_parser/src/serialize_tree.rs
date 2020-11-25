use indextree::{NodeId, Arena};
use super::*;
use serde::{Serialize, Serializer};
use serde::ser::{SerializeSeq};


#[derive(Serialize)]
pub struct SerializeNode<'a> (
    &'a RaBlock, 
    #[serde(skip_serializing_if = "Option::is_none")]
    Option<SerializeTree<'a>>
);

impl<'a> SerializeNode<'a> {
    pub fn new(id: NodeId, arena: &'a Arena<RaBlock>) -> Self {
        let node = &arena[id];
        SerializeNode (&node.get(), node
        .first_child()
        .map(|first| SerializeTree::new(first, arena)))
    }
}

/// Convenience wrapper struct for serializing a node and its siblings.
pub struct SerializeTree<'a> {
    first: NodeId,
    arena: &'a Arena<RaBlock>,
}

impl<'a> SerializeTree<'a> {
    pub fn new(id: NodeId, arena: &'a Arena<RaBlock>) -> Self {
        SerializeTree { first: id, arena }
    }
}

impl Serialize for SerializeTree<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(None)?;
        for node in self.first.following_siblings(&self.arena) {
            seq.serialize_element(&SerializeNode::new(node, &self.arena))?;
        }
        seq.end()
    }
}