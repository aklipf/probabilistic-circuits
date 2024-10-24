pub mod builder;
pub mod node;

#[cfg(test)]
mod tests;

pub use builder::*;
pub use node::*;

use crate::tree::{Addr, Node, Tree};

use super::semantic::Semantic;

pub use FOLogic as FirstOrderLogic;
pub type FirstOrderTree = Tree<FOLogic, 2>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FOLogic {
    Predicate { id: Addr },
    Universal { id: Addr },
    Existential { id: Addr },
    Not,
    And,
    Or,
}

impl Semantic for FOLogic {
    type Tree = Tree<FOLogic, 2>;
    type Node = Node<2>;
}
