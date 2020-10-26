use super::*;
use std::convert::TryInto;

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

        traverser.next_child();

        traverser
    }
    

    pub fn next_child(&mut self) {
        self.address.push(0);
        self.head.read_at(&self.address);
    }

    pub fn next_sibling(&mut self) {
        match self.address.pop() {
            Some(last_index) => {
                self.address.push(last_index + 1)
            },
            None => {
                self.address.push(0);
            }
        };

        self.head.read_at(&self.address);
    }
    
}

