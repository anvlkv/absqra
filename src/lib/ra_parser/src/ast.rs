use super::*;
use serde::ser::{Serialize, Serializer, SerializeStruct};

#[derive(Debug, Clone)]
pub struct RaAST {
    node: RaASTNode,
    children: Vec<Box<RaAST>>,
    position: (Position, Position),
    level: u16,


    /// non-serializable field parent
    parent: Option<Box<RaAST>>,
    /// non-serializable field root
    root: Option<Box<RaAST>>,
}

impl RaAST {
    pub fn new() -> Self {
        Self {
            node: RaASTNode::Root,
            children: Vec::new(),
            position: (Position::default(), Position::default()),
            level: 0,
            parent: None,
            root: None
        }
    }
}

impl Serialize for RaAST {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("RaAST", 2)?;
        state.serialize_field("node", &self.node)?;
        state.serialize_field("children", &self.children.iter().map(|cr| cr.as_ref()).collect::<Vec<&RaAST>>())?;
        state.serialize_field("position", &self.position)?;
        state.serialize_field("level", &self.level)?;
        state.end()
    }
}

impl From<RaTree> for RaAST {
    fn from(tree: RaTree) -> Self {
        let mut ast = Self::new();
        let mut cursor = Cursor::new(tree);

        

        ast
    }
}
