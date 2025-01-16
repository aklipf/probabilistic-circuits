pub mod builder;
pub mod compile;
pub mod eval;
pub mod node;

#[cfg(test)]
mod tests;

pub use builder::*;
pub use compile::*;
pub use node::*;

use crate::tree::{Addr, Node, Tree};

use super::Semantic;

pub use PCicruit as ProbabilisticCircuit;
pub type ProbabilisticCircuitTree = Tree<PCicruit, 2>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCicruit {
    Variable { id: Addr, neg: bool },
    Product,
    Sum { left: f32, right: f32 },
}

impl Semantic for PCicruit {
    type Tree = Tree<PCicruit, 2>;
    type Node = Node<2>;
}
