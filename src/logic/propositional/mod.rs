pub mod builder;
pub mod dnf;
pub mod eval;
pub mod nnf;
pub mod node;

#[cfg(test)]
mod tests;

pub use builder::*;
pub use dnf::*;
pub use nnf::*;
pub use node::*;

use super::semantic::Semantic;

use crate::tree::{Addr, Node, Tree};

pub use PLogic as PropositionalLogic;
pub type PropositionalTree = Tree<PropositionalLogic, 2>;

pub use propositional_to_nnf;
pub use PMut;
pub use PRef;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLogic {
    Variable { id: Addr },
    Not,
    And,
    Or,
}

impl Semantic for PLogic {
    type Tree = Tree<PLogic, 2>;
    type Node = Node<2>;
}
