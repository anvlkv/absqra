use super::*;
// use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::convert::TryFrom;

#[derive(Debug, Clone, Serialize)]
pub struct RaAST {
    node: RaASTNode,
    children: Vec<Box<RaAST>>,
    position: (Position, Position),
    level: u16,

    // /// non-serializable field parent
    // parent: Option<Box<RaAST>>,
    // /// non-serializable field root
    // root: Option<Box<RaAST>>,
}

impl RaAST {
    pub (crate) fn new() -> Self {
        Self {
            node: RaASTNode::Root,
            children: Vec::new(),
            position: (Position::default(), Position::default()),
            level: 0,
            // parent: None,
            // root: None,
        }
    }

    pub (crate) fn read(&mut self, mut traverser: &BlockTreeTraverser) -> Result<(), Vec<ParserError>> {
        match traverser.head.current {
            Some(tokens) => {
                let mut ast = self.parse(tokens)?;
                // ast.children.
                self.children.push(Box::new(ast));
                Ok(())
            },
            None => {
                Err(vec![ParserError::UnexpectedEndOfInput(self.position.1, Backtrace::new())])
            }
        }
    }

    fn parse(&self, tokens: &[RaToken]) -> Result<Self, Vec<ParserError>> {
        let node = RaASTNode::parse(tokens)?;
        let position = node.position(); 
        let level = node.level();

        Ok(Self {
            node,
            position,
            level,
            // root: Some(Box::from(self)),
            // parent: Some(Box::from(self)),
            children: Vec::new()
        })
    }
}

// impl Serialize for RaAST {
//     fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
//     where
//         S: Serializer,
//     {
//         let mut state = serializer.serialize_struct("RaAST", 2)?;
//         state.serialize_field("node", &self.node)?;
//         state.serialize_field(
//             "children",
//             &self
//                 .children
//                 .iter()
//                 .map(|cr| cr.as_ref())
//                 .collect::<Vec<&RaAST>>(),
//         )?;
//         state.serialize_field("position", &self.position)?;
//         state.serialize_field("level", &self.level)?;
//         state.end()
//     }
// }

impl TryFrom<RaTree> for RaAST {
    type Error = Vec<ParserError>;

    fn try_from(tree: RaTree) -> Result<Self, Vec<ParserError>> {
        let mut ast = Self::new();
        let meaningful_tree = tree.no_comments();
        let mut traverser = BlockTreeTraverser::new(&meaningful_tree);

        while traverser.head.current.is_some() {
            ast.read(&traverser)?;
            traverser.next_sibling();
        }

        Ok(ast)
    }
}