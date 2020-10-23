use super::*;

pub (crate) type TreeAddress = Vec<u16>; 

pub (crate) struct BlockTreeTraverser<'t> {
    pub head: Head<'t>,
    address: TreeAddress
}


impl<'t> BlockTreeTraverser<'t> {
    pub fn new(tree: &'t RaTree) -> Self {

        let mut traverser = Self {
            head: Head::new(tree),
            address: vec![]
        };

        traverser.advance_block();

        traverser
    }
    

    pub fn advance_block(&mut self) {
        let mut address = self.address.clone();

        let last_index = address.pop();

        match last_index {
            Some(last_index) => {
                let parent = self.position_head().unwrap();
                match parent.children.iter().nth(usize::from(last_index + 1)) {
                    Some(next) => {
                        self.head.current = Some(&next.tokens);
                        address.push(last_index + 1);
                    },
                    None => {
                        if address.len() >= 1 {
                            self.advance_block();
                        }
                        else {
                            self.address = vec![];
                            self.head.current = None;
                        }
                    }
                }
            },
            None => {
                self.address = vec![0];
                let first = self.position_head();
                match first {
                    Some(next) => {
                        self.head.current = Some(&next.tokens);
                    },
                    None => {}
                }
            }
        }
    }

    fn position_head(&self) -> Option<&'t RaTree> {
        let mut tree_address_iter = self.address.iter();
        let mut last_node = Some(self.head.tree);

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

