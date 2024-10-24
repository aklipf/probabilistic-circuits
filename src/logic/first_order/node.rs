use std::{fmt::Display, ops::Index};

use crate::{
    logic::{Semantic, SemanticNode},
    tree::{Addr, IndexedRef, LinkingNode, Mapping, Node, NodeValue, Tree},
};

use super::FOLogic;

pub trait FORef {
    fn left(&self) -> Self;
    fn right(&self) -> Self;
    fn inner(&self) -> Self;
}

impl<'a, T> FORef for IndexedRef<'a, T>
where
    T: Index<Addr, Output = NodeValue<Node<2>, FOLogic>>,
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

impl Display for Tree<FOLogic, 2> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.output(), f)
    }
}

impl<'a> Display for IndexedRef<'a, <FOLogic as Semantic>::Tree> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.as_ref().value {
            FOLogic::Predicate { id } => {
                write!(f, "{}(", self.array.fmt_named(id))?;

                let mut first_var = true;
                let mut node_id = self.inner();
                if node_id.idx.is_none() {
                    return write!(f, ")");
                }

                while node_id.idx.is_addr() {
                    if first_var {
                        first_var = false;
                    } else {
                        write!(f, ", ")?;
                    }

                    match node_id.as_ref().value {
                        FOLogic::Predicate { id: var_id } => {
                            write!(f, "{}", self.array.fmt_named(var_id))
                        }
                        _ => panic!(),
                    }?;

                    node_id = node_id.inner();
                }
                write!(f, ")")
            }
            FOLogic::Not => {
                write!(f, "\u{00AC}")?;
                Display::fmt(&self.inner(), f)
            }
            FOLogic::And => {
                write!(f, "(")?;
                Display::fmt(&self.left(), f)?;
                write!(f, "\u{2227}")?;
                Display::fmt(&self.right(), f)?;
                write!(f, ")")
            }
            FOLogic::Or => {
                write!(f, "(")?;
                Display::fmt(&self.left(), f)?;
                write!(f, "\u{2228}")?;
                Display::fmt(&self.right(), f)?;
                write!(f, ")")
            }
            FOLogic::Universal { id } => {
                write!(f, "\u{2200}{}:", self.array.fmt_named(id))?;
                Display::fmt(&self.inner(), f)
            }
            FOLogic::Existential { id } => {
                write!(f, "\u{2203}{}:", self.array.fmt_named(id))?;
                Display::fmt(&self.inner(), f)
            }
        }
    }
}

impl SemanticNode for NodeValue<Node<2>, FOLogic> {
    fn arity(&self) -> usize {
        match self.value {
            FOLogic::Predicate { id: _ } => 0,
            FOLogic::Universal { id: _ } => 1,
            FOLogic::Existential { id: _ } => 1,
            FOLogic::Not => 1,
            FOLogic::And => 2,
            FOLogic::Or => 2,
        }
    }
}
