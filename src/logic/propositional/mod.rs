pub mod builder;
pub mod eval;
pub mod node;

#[cfg(test)]
mod tests;

pub use builder::*;
pub use eval::*;
pub use node::*;

use super::fragment::Fragment;

use crate::tree::{Addr, Node, Tree};

pub use PLogic as PropositionalLogic;
pub type PropositionalTree = Tree<PropositionalLogic, 2>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLogic {
    Variable { id: Addr },
    Not,
    And,
    Or,
}

impl Fragment for PLogic {
    type Tree = Tree<PLogic, 2>;
    type Node = Node<2>;
}
