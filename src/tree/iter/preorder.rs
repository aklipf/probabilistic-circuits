use crate::tree::index::Indexing;

use super::super::node::Node;
use super::super::tree::Tree;

pub trait IntoPreorderIterator {
    type Iter;
    fn iter_preorder(self) -> Self::Iter;
}

impl<'a, IDX: Indexing> IntoPreorderIterator for &'a Tree<IDX> {
    type Iter = IterPreorder<'a, IDX>;
    fn iter_preorder(self) -> Self::Iter {
        let first = self.output;
        IterPreorder {
            tree: self,
            stack_idx: vec![first],
        }
    }
}

pub struct IterPreorder<'a, IDX: Indexing> {
    tree: &'a Tree<IDX>,
    stack_idx: Vec<IDX>,
}

impl<'a, IDX: Indexing> Iterator for IterPreorder<'a, IDX> {
    type Item = &'a Node<IDX>;
    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.stack_idx.pop()?;
        let childs = self.tree.nodes[idx.addr()].childs();
        self.stack_idx.extend_from_slice(childs);
        Some(&self.tree.nodes[idx.addr()])
    }
}
