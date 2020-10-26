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

    pub fn read_at(&mut self, address: &TreeAddress) {
        let mut tree_address_iter = address.iter();
        let mut last_node = Some(self.tree);

        while let Some(i) = tree_address_iter.next() {
            match last_node {
                Some(node) => {
                    last_node = node.children.iter().map(|b| b.as_ref()).nth(usize::from(*i));
                },
                None => {
                    break;
                }
            }
        }

        self.current = {
            match last_node {
                Some(n) => Some(&n.tokens),
                None => None
            }
        };
    }

}