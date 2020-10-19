use super::*;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::convert::TryFrom;

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
    pub (crate) fn new() -> Self {
        Self {
            node: RaASTNode::Root,
            children: Vec::new(),
            position: (Position::default(), Position::default()),
            level: 0,
            parent: None,
            root: None,
        }
    }

    pub (crate) fn read(&mut self, mut traverser: &BlockTreeTraverser, mut address: &TreeAddress) -> Result<(), Vec<ParserError>> {
        
        todo!();
    }
}

impl Serialize for RaAST {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("RaAST", 2)?;
        state.serialize_field("node", &self.node)?;
        state.serialize_field(
            "children",
            &self
                .children
                .iter()
                .map(|cr| cr.as_ref())
                .collect::<Vec<&RaAST>>(),
        )?;
        state.serialize_field("position", &self.position)?;
        state.serialize_field("level", &self.level)?;
        state.end()
    }
}

impl TryFrom<RaTree> for RaAST {
    type Error = Vec<ParserError>;

    fn try_from(tree: RaTree) -> Result<Self, Vec<ParserError>> {
        let mut ast = Self::new();
        let (mut traverser, mut address) = BlockTreeTraverser::new_with_address(&tree);

        while traverser.current.is_some() {
            ast.read(&traverser, &address)?;
            traverser.advance_block(&address);
        }

        Ok(ast)
    }
}
/*


*/