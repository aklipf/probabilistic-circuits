use std::ops::Index;

use crate::tree::{
    index::Indexing,
    tree::{ExpressionTree, Tree},
    view::NodeView,
};

use super::{node::PropositionalNode, PropositionalLogic};

pub trait Eval<D> {
    fn eval(&self, assignment: &Vec<D>) -> bool;
}

fn recursive_eval<'a, I: Indexing>(
    node: &NodeView<'a, PropositionalLogic<I>, I, 2>,
    assignment: &Vec<bool>,
) -> bool {
    match node.symbol() {
        PropositionalLogic::Variable { id } => assignment[id.addr()],
        PropositionalLogic::Not => !recursive_eval(&node.input(), assignment),
        PropositionalLogic::And => {
            recursive_eval(&node.left(), assignment) && recursive_eval(&node.right(), assignment)
        }
        PropositionalLogic::Or => {
            recursive_eval(&node.left(), assignment) || recursive_eval(&node.right(), assignment)
        }
        _ => panic!(),
    }
}

impl<I> Eval<bool> for Tree<PropositionalLogic<I>, I, 2>
where
    I: Indexing,
    Tree<PropositionalLogic<I>, I, 2>:
        Index<I, Output = PropositionalNode<I>> + ExpressionTree<PropositionalLogic<I>, I, 2>,
{
    fn eval(&self, assignment: &Vec<bool>) -> bool {
        recursive_eval(&self.output(), assignment)
    }
}
