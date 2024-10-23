pub mod builder;
pub mod ground;
pub mod node;

use super::fragment::Fragment;
use crate::tree::index::Indexing;
use node::FirstOrderNode;

#[derive(Clone, Copy, Debug)]
pub enum FirstOrderLogic<IDX: Indexing = u32> {
    Predicate { id: IDX },
    Universal { id: IDX },
    Existential { id: IDX },
    Not,
    And,
    Or,
    None,
}

impl<I: Indexing> Fragment<I, 2> for FirstOrderLogic<I> {
    type Node = FirstOrderNode<I>;
}

impl<IDX: Indexing> Default for FirstOrderLogic<IDX> {
    fn default() -> Self {
        FirstOrderLogic::None
    }
}
