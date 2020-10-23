use super::*;

#[derive(Copy, Clone)]
pub (crate) struct Head<'t> {
    pub tree: &'t RaTree,
    pub level: u16,
    pub position_span: (Position, Position),
    pub current: Option<&'t Vec<RaToken>>
}

impl<'t> Head<'t> {
    pub fn new(tree: &'t RaTree) -> Self {
        let level = tree.level;
        let position_span = tree.position;
    
        Self {
            tree,
            level,
            position_span,
            current: None
        }
    }

}