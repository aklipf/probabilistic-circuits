use crate::wfol::index::Indexing;

use super::super::tree::{Node, Tree};

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

        match &self.tree.nodes[idx.addr()] {
            Node::Not { inputs, .. } => self.stack_idx.push(inputs[0]),
            Node::And { inputs, .. } => {
                self.stack_idx.push(inputs[0]);
                self.stack_idx.push(inputs[1])
            }
            Node::Or { inputs, .. } => {
                self.stack_idx.push(inputs[0]);
                self.stack_idx.push(inputs[1])
            }
            Node::All {
                var_id: _, inputs, ..
            } => self.stack_idx.push(inputs[0]),
            Node::Any {
                var_id: _, inputs, ..
            } => self.stack_idx.push(inputs[0]),
            _ => {}
        }
        Some(&self.tree.nodes[idx.addr()])
    }
}
