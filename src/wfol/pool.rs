use std::ops::{Index, IndexMut};

use super::tree::Tree;

use super::{index::Indexing, node::Node};

pub trait Pool:
    Index<Self::IDX, Output = Node<Self::IDX>> + IndexMut<Self::IDX, Output = Node<Self::IDX>>
where
    Self::IDX: Indexing,
{
    type IDX;
    fn push(&mut self, node: Node<Self::IDX>) -> Self::IDX;
}

impl<IDX: Indexing> Pool for Tree<IDX> {
    type IDX = IDX;

    fn push(&mut self, node: Node<IDX>) -> IDX {
        let idx = IDX::from(self.nodes.len());
        self.nodes.push(node);
        idx
    }
}
