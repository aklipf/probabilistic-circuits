pub mod builder;
pub mod eval;
pub mod node;

use node::PropositionalNode;

use crate::tree::index::Indexing;

use super::fragment::Fragment;

#[derive(Clone, Copy, Debug)]
pub enum PropositionalLogic<IDX: Indexing = u32> {
    Variable { id: IDX },
    Not,
    And,
    Or,
    None,
}

impl<I: Indexing> Fragment<I, 2> for PropositionalLogic<I> {
    type Node = PropositionalNode<I>;
}

impl<IDX: Indexing> Default for PropositionalLogic<IDX> {
    fn default() -> Self {
        PropositionalLogic::None
    }
}
