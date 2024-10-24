use std::{fmt::Display, ops::Index};

use crate::{
    logic::{Semantic, SemanticNode},
    tree::{Addr, IndexedRef, LinkingNode, Mapping, Node, NodeValue, Tree},
};

use super::PCicruit;

pub trait PCRef {
    fn left(&self) -> Self;
    fn right(&self) -> Self;
}

impl<'a, T> PCRef for IndexedRef<'a, T>
where
    T: Index<Addr, Output = NodeValue<Node<2>, PCicruit>>,
{
    fn left(&self) -> Self {
        IndexedRef {
            array: &self.array,
            idx: self.as_ref().node.operands()[0],
        }
    }

    fn right(&self) -> Self {
        IndexedRef {
            array: &self.array,
            idx: self.as_ref().node.operands()[1],
        }
    }
}

impl Display for Tree<PCicruit, 2> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.output(), f)
    }
}

impl<'a> Display for IndexedRef<'a, <PCicruit as Semantic>::Tree> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.as_ref().value {
            PCicruit::Variable { id, neg } => {
                write!(
                    f,
                    "{}{}",
                    if neg { "\u{00AC}" } else { "" },
                    self.array.fmt_named(id)
                )
            }
            PCicruit::Product => {
                write!(f, "(")?;
                Display::fmt(&self.left(), f)?;
                write!(f, "*")?;
                Display::fmt(&self.right(), f)?;
                write!(f, ")")
            }
            PCicruit::Sum { left, right } => {
                write!(f, "(")?;
                if left != 1.0 {
                    write!(f, "{left:.3}\u{2219}")?;
                }
                Display::fmt(&self.left(), f)?;
                write!(f, "+")?;
                if right != 1.0 {
                    write!(f, "{right:.3}\u{2219}")?;
                }
                Display::fmt(&self.right(), f)?;
                write!(f, ")")
            }
        }
    }
}

impl SemanticNode for NodeValue<Node<2>, PCicruit> {
    fn arity(&self) -> usize {
        match self.value {
            PCicruit::Variable { .. } => 0,
            PCicruit::Product => 2,
            PCicruit::Sum { .. } => 2,
        }
    }
}
