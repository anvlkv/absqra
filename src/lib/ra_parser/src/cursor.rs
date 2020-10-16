use super::*;

pub (crate) struct Cursor {
    tree: RaTree,
    pub level: u16,
    pub position_span: (Position, Position)
}

impl Cursor {
    pub fn new(tree: RaTree) -> Self {
        Self {
            tree,
            level: tree.level,
            position_span: tree.position
        }
    }
}