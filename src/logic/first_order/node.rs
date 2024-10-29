use std::{fmt::Display, ops::Index};

use crate::{
    logic::{Semantic, SemanticNode},
    tree::{Addr, IndexedRef, LinkingNode, Mapping, Node, NodeValue, Tree},
};

use super::FOLogic;

pub trait FORef: Sized {
    fn left(&self) -> Option<Self>;
    fn right(&self) -> Option<Self>;
    fn inner(&self) -> Option<Self>;
}

pub trait Args<'a, T: Index<Addr>> {
    fn args(&self) -> ArgsIter<'a, T>;
}

pub struct ArgsIter<'a, T>
where
    T: Index<Addr>,
{
    pub array: &'a T,
    pub idx: Addr,
}

impl<'a, T> Args<'a, T> for IndexedRef<'a, T>
where
    T: Index<Addr, Output = NodeValue<Node<2>, FOLogic>>,
{
    fn args(&self) -> ArgsIter<'a, T> {
        ArgsIter {
            array: &self.array,
            idx: self.array[self.idx].node.operands()[0],
        }
    }
}

impl<'a, T> Iterator for ArgsIter<'a, T>
where
    T: Index<Addr, Output = NodeValue<Node<2>, FOLogic>>,
{
    type Item = Addr;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx.is_none() {
            return None;
        }
        let id = if let FOLogic::Predicate { id } = self.array[self.idx].value {
            id
        } else {
            panic!()
        };

        self.idx = self.array[self.idx].node.operands()[0];

        Some(id)
    }
}

impl<'a, T> FORef for IndexedRef<'a, T>
where
    T: Index<Addr, Output = NodeValue<Node<2>, FOLogic>>,
{
    fn left(&self) -> Option<Self> {
        let child_addr = self.as_ref().node.operands()[0];
        if child_addr.is_addr() {
            Some(IndexedRef {
                array: &self.array,
                idx: child_addr,
            })
        } else {
            None
        }
    }

    fn right(&self) -> Option<Self> {
        let child_addr = self.as_ref().node.operands()[1];
        if child_addr.is_addr() {
            Some(IndexedRef {
                array: &self.array,
                idx: child_addr,
            })
        } else {
            None
        }
    }

    fn inner(&self) -> Option<Self> {
        let child_addr = self.as_ref().node.operands()[0];
        if child_addr.is_addr() {
            Some(IndexedRef {
                array: &self.array,
                idx: child_addr,
            })
        } else {
            None
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

                while let Some(node) = node_id {
                    if first_var {
                        first_var = false;
                    } else {
                        write!(f, ", ")?;
                    }

                    match node.as_ref().value {
                        FOLogic::Predicate { id: var_id } => {
                            write!(f, "{}", self.array.fmt_named(var_id))
                        }
                        _ => panic!(),
                    }?;

                    node_id = node.inner();
                }
                write!(f, ")")
            }
            FOLogic::Not => {
                write!(f, "\u{00AC}")?;
                Display::fmt(&self.inner().unwrap(), f)
            }
            FOLogic::And => {
                write!(f, "(")?;
                Display::fmt(&self.left().unwrap(), f)?;
                write!(f, "\u{2227}")?;
                Display::fmt(&self.right().unwrap(), f)?;
                write!(f, ")")
            }
            FOLogic::Or => {
                write!(f, "(")?;
                Display::fmt(&self.left().unwrap(), f)?;
                write!(f, "\u{2228}")?;
                Display::fmt(&self.right().unwrap(), f)?;
                write!(f, ")")
            }
            FOLogic::Universal { id } => {
                write!(f, "\u{2200}{}:", self.array.fmt_named(id))?;
                Display::fmt(&self.inner().unwrap(), f)
            }
            FOLogic::Existential { id } => {
                write!(f, "\u{2203}{}:", self.array.fmt_named(id))?;
                Display::fmt(&self.inner().unwrap(), f)
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
