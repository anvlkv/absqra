use super::*;

pub (crate) type TreeAddress = Vec<u16>; 

#[derive(Copy, Clone)]
pub (crate) struct BlockTreeTraverser<'t> {
    tree: &'t RaTree,
    pub level: u16,
    pub position_span: (Position, Position),
    pub current: Option<&'t Vec<RaToken>>
}

impl<'t> BlockTreeTraverser<'t> {
    pub fn new(tree: &'t RaTree) -> Self {
        let level = tree.level;
        let position_span = tree.position.clone();

        Self {
            tree,
            level,
            position_span,
            current: None
        }

    }
    
    pub fn new_with_address(tree: &'t RaTree) -> (Self, TreeAddress) {
        let mut traverser = Self::new(tree);
        let mut address = vec![];
    
        traverser.advance_block(&address);
    
        (traverser, address)
    }

    pub fn advance_block(&mut self, mut address: &TreeAddress) {
        // let mut address = self.tree_address.clone();

        let last_index = address.pop();

        match last_index {
            Some(last_index) => {
                let parent = self.get_at_address(&address).unwrap();
                match parent.children.iter().nth(usize::from(last_index + 1)) {
                    Some(next) => {
                        self.current = Some(&next.tokens);
                        address.push(last_index + 1);
                    },
                    None => {
                        if address.len() >= 1 {
                            self.advance_block(address);
                        }
                        else {
                            address = vec![];
                            self.current = None;
                        }
                    }
                }
            },
            None => {
                let first = self.get_at_address(&vec![0]);
                match first {
                    Some(next) => {
                        self.current = Some(&next.tokens);
                        address = vec![0]
                    },
                    None => {}
                }
            }
        }
    }

    fn get_at_address(&self, address: &TreeAddress) -> Option<&'t RaTree> {
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

        last_node
    }
    
}

