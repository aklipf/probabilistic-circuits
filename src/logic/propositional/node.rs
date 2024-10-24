use std::{fmt::Display, ops::Index};

use crate::tree::{Addr, IndexedRef, LinkingNode, Mapping, Node, NodeValue, Tree};

use super::{super::fragment::FragmentNode, PLogic};

pub trait PRef {
    fn left(&self) -> Self;
    fn right(&self) -> Self;
    fn inner(&self) -> Self;
}

impl<'a, T> PRef for IndexedRef<'a, T>
where
    T: Index<Addr, Output = NodeValue<Node<2>, PLogic>>,
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

    fn inner(&self) -> Self {
        IndexedRef {
            array: &self.array,
            idx: self.as_ref().node.operands()[0],
        }
    }
}

impl Display for Tree<PLogic, 2> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.output(), f)
    }
}

impl<'a, T> Display for IndexedRef<'a, T>
where
    T: Index<Addr, Output = NodeValue<Node<2>, PLogic>> + Mapping,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.as_ref().value {
            PLogic::Variable { id } => {
                write!(
                    f,
                    "{}",
                    self.array
                        .get_named(id)
                        .unwrap_or(&format!("Anon{}", id.addr()))
                )
            }
            PLogic::Not => {
                write!(f, "\u{00AC}")?;
                Display::fmt(&self.inner(), f)
            }
            PLogic::And => {
                write!(f, "(")?;
                Display::fmt(&self.left(), f)?;
                write!(f, "\u{2227}")?;
                Display::fmt(&self.right(), f)?;
                write!(f, ")")
            }
            PLogic::Or => {
                write!(f, "(")?;
                Display::fmt(&self.left(), f)?;
                write!(f, "\u{2228}")?;
                Display::fmt(&self.right(), f)?;
                write!(f, ")")
            }
        }
    }
}

impl FragmentNode for NodeValue<Node<2>, PLogic> {
    fn arity(&self) -> usize {
        match self.value {
            PLogic::Variable { id: _ } => 0,
            PLogic::Not => 1,
            PLogic::And => 2,
            PLogic::Or => 2,
        }
    }
}
