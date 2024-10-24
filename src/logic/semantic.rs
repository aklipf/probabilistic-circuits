use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Index;

use crate::tree::{Addr, LinkingNode, NodeValue};

pub trait SemanticNode {
    fn arity(&self) -> usize;
}

pub trait Semantic: Clone + Copy + Debug + PartialEq
where
    NodeValue<Self::Node, Self>: SemanticNode,
{
    type Tree: Index<Addr, Output = NodeValue<Self::Node, Self>> + Display;
    type Node: LinkingNode;

    fn symbol(&self) -> Self {
        *self
    }
}

pub trait Eval<D> {
    type Output;

    fn eval(&self, assignment: &Vec<D>) -> Self::Output;
}
