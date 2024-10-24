use std::ops::Index;

use crate::tree::{Addr, IndexedRef, Node, NodeValue, Tree};

use super::{PLogic, PRef};

pub trait Eval<D> {
    fn eval(&self, assignment: &Vec<D>) -> bool;
}

impl<'a, T> Eval<bool> for IndexedRef<'a, T>
where
    T: Index<Addr, Output = NodeValue<Node<2>, PLogic>>,
{
    fn eval(&self, assignment: &Vec<bool>) -> bool {
        match self.as_ref().value {
            PLogic::Variable { id } => assignment[id.addr()],
            PLogic::Not => !self.inner().eval(assignment),
            PLogic::And => self.left().eval(assignment) && self.right().eval(assignment),
            PLogic::Or => self.left().eval(assignment) || self.right().eval(assignment),
        }
    }
}

impl Eval<bool> for Tree<PLogic, 2>
where
    Tree<PLogic, 2>: Index<Addr, Output = NodeValue<Node<2>, PLogic>>,
{
    fn eval(&self, assignment: &Vec<bool>) -> bool {
        self.output().eval(assignment)
    }
}
