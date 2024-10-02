use std::ops::{Index, IndexMut};

use crate::tree::{
    allocator::Allocator, builder::Builder, index::Indexing, mapping::Mapping, node::Node,
    tree::Tree,
};

use super::fragment::{Fragment, OperandsIter, Symbols};

#[derive(Clone, Copy, Debug)]
pub enum PropSymbols<IDX: Indexing = u32> {
    Variable { id: IDX },
    Not,
    And,
    Or,
    None,
}

impl<IDX: Indexing> Symbols for PropSymbols<IDX> {}

impl<IDX: Indexing> Default for PropSymbols<IDX> {
    fn default() -> Self {
        PropSymbols::None
    }
}

pub trait PropositionalLogic<I: Indexing> {
    fn var(&mut self, id: I) -> I;
    fn not<F: Fn(&mut Self) -> I>(&mut self, inner: F) -> I;
    fn and<F: Fn(&mut Self) -> I, G: Fn(&mut Self) -> I>(&mut self, left: F, right: G) -> I;
    fn or<F: Fn(&mut Self) -> I, G: Fn(&mut Self) -> I>(&mut self, left: F, right: G) -> I;
}

pub type PropNode<IDX> = Node<IDX, PropSymbols<IDX>, 2>;

impl<IDX: Indexing> Fragment<IDX> for PropNode<IDX> {
    type Symbols = PropSymbols<IDX>;

    fn fmt_display<T: Mapping<IDX = IDX> + Index<IDX, Output = Self>>(
        &self,
        f: &mut std::fmt::Formatter,
        tree: &T,
    ) -> std::fmt::Result {
        match self.symbol {
            PropSymbols::Variable { id } => {
                write!(
                    f,
                    "{}",
                    tree.get_named(id).unwrap_or(&format!("Anon{}", id.addr()))
                )
            }
            PropSymbols::Not => {
                write!(f, "\u{00AC}")?;
                self.operands(tree).next().unwrap().fmt_display(f, tree)
            }
            PropSymbols::And => {
                let mut operands = self.operands(tree);
                write!(f, "(")?;
                operands.next().unwrap().fmt_display(f, tree)?;
                write!(f, "\u{2227}")?;
                operands.next().unwrap().fmt_display(f, tree)?;
                write!(f, ")")
            }
            PropSymbols::Or => {
                let mut operands = self.operands(tree);
                write!(f, "(")?;
                operands.next().unwrap().fmt_display(f, tree)?;
                write!(f, "\u{2228}")?;
                operands.next().unwrap().fmt_display(f, tree)?;
                write!(f, ")")
            }
            PropSymbols::None => panic!("Can't display a partially initialised tree."),
        }
    }

    fn arity(&self) -> usize {
        match self.symbol {
            PropSymbols::Variable { id: _ } => 0,
            PropSymbols::Not => 1,
            PropSymbols::And => 2,
            PropSymbols::Or => 2,
            PropSymbols::None => 0,
        }
    }
}

impl<'a, I, P> PropositionalLogic<I> for Builder<'a, I, PropSymbols<I>, 2, P>
where
    I: Indexing,
    P: Allocator<IDX = I, Symbols = PropSymbols<I>>
        + Mapping<IDX = I>
        + Index<I, Output = Node<I, PropSymbols<I>, 2>>
        + IndexMut<I>,
{
    #[inline(always)]
    fn var(&mut self, id: I) -> I {
        self.push(PropSymbols::Variable { id: id }, &[])
    }

    #[inline(always)]
    fn not<F: Fn(&mut Self) -> I>(&mut self, inner: F) -> I {
        let inner_id = inner(self);
        self.push(PropSymbols::Not, &[inner_id])
    }

    #[inline(always)]
    fn and<F: Fn(&mut Self) -> I, G: Fn(&mut Self) -> I>(&mut self, left: F, right: G) -> I {
        let left_id = left(self);
        let right_id = right(self);
        self.push(PropSymbols::And, &[left_id, right_id])
    }

    #[inline(always)]
    fn or<F: Fn(&mut Self) -> I, G: Fn(&mut Self) -> I>(&mut self, left: F, right: G) -> I {
        let left_id = left(self);
        let right_id = right(self);
        self.push(PropSymbols::Or, &[left_id, right_id])
    }
}
