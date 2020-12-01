use indextree::{NodeId, Arena};
use serde::{Serialize, Serializer};
use serde::ser::{SerializeSeq};


#[derive(Serialize)]
pub struct SerializeNode<'a, T>(
    &'a T, 
    #[serde(skip_serializing_if = "Option::is_none")]
    Option<SerializeTree<'a, T>>
);

impl<'a, T> SerializeNode<'a, T> {
    pub fn new(id: NodeId, arena: &'a Arena<T>) -> Self {
        let node = &arena[id];
        SerializeNode (&node.get(), node
        .first_child()
        .map(|first| SerializeTree::new(first, arena)))
    }
}

pub struct SerializeTree<'a, T> {
    first: NodeId,
    arena: &'a Arena<T>,
}

impl<'a, T> SerializeTree<'a, T> {
    pub fn new(id: NodeId, arena: &'a Arena<T>) -> Self {
        SerializeTree { first: id, arena }
    }
}

impl<T> Serialize for SerializeTree<'_, T> where T: Serialize {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(None)?;
        for node in self.first.following_siblings(&self.arena) {
            seq.serialize_element(&SerializeNode::new(node, &self.arena))?;
        }
        seq.end()
    }
}