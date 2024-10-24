use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Index;

use crate::tree::{Addr, LinkingNode, NodeValue};

pub trait FragmentNode {
    fn arity(&self) -> usize;
}

pub trait Fragment: Clone + Copy + Debug + PartialEq
where
    NodeValue<Self::Node, Self>: FragmentNode,
{
    type Tree: Index<Addr, Output = NodeValue<Self::Node, Self>> + Display;
    type Node: LinkingNode;

    fn symbol(&self) -> Self {
        *self
    }
}
