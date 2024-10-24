pub mod builder;
pub mod node;

use crate::tree::{Addr, Node, Tree};

pub use builder::*;
pub use node::*;

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
