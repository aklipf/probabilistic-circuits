use std::ops::Index;

use crate::tree::{
    builder::{Buildable, Builder},
    index::Indexing,
    mapping::Mapping,
    node::{LinkinNode, Node},
};

use super::fragment::{Fragment, FragmentNode};

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

pub trait PropositionalLogicBuilder<I: Indexing> {
    fn var(&mut self, id: I) -> I;
    fn not<F: Fn(&mut Self) -> I>(&mut self, inner: F) -> I;
    fn and<F: Fn(&mut Self) -> I, G: Fn(&mut Self) -> I>(&mut self, left: F, right: G) -> I;
    fn or<F: Fn(&mut Self) -> I, G: Fn(&mut Self) -> I>(&mut self, left: F, right: G) -> I;
}

pub type PropositionalNode<I> = Node<I, PropositionalLogic<I>, 2>;

impl<I: Indexing> FragmentNode<I, PropositionalLogic<I>, 2> for PropositionalNode<I> {
    fn fmt_display<T: Mapping<I> + Index<I, Output = Self>>(
        &self,
        f: &mut std::fmt::Formatter,
        tree: &T,
    ) -> std::fmt::Result {
        match self.symbol() {
            PropositionalLogic::Variable { id } => {
                write!(
                    f,
                    "{}",
                    tree.get_named(id).unwrap_or(&format!("Anon{}", id.addr()))
                )
            }
            PropositionalLogic::Not => {
                write!(f, "\u{00AC}")?;
                tree[self.operands().next().unwrap()].fmt_display(f, tree)
            }
            PropositionalLogic::And => {
                let mut operands = self.operands();
                write!(f, "(")?;
                tree[operands.next().unwrap()].fmt_display(f, tree)?;
                write!(f, "\u{2227}")?;
                tree[operands.next().unwrap()].fmt_display(f, tree)?;
                write!(f, ")")
            }
            PropositionalLogic::Or => {
                let mut operands = self.operands();
                write!(f, "(")?;
                tree[operands.next().unwrap()].fmt_display(f, tree)?;
                write!(f, "\u{2228}")?;
                tree[operands.next().unwrap()].fmt_display(f, tree)?;
                write!(f, ")")
            }
            PropositionalLogic::None => panic!("Can't display a partially initialised tree."),
        }
    }

    fn arity(&self) -> usize {
        match self.symbol() {
            PropositionalLogic::Variable { id: _ } => 0,
            PropositionalLogic::Not => 1,
            PropositionalLogic::And => 2,
            PropositionalLogic::Or => 2,
            PropositionalLogic::None => 0,
        }
    }

    fn new(symbol: PropositionalLogic<I>, operands: &[I]) -> Self {
        Self::new(symbol, operands)
    }

    fn duplicate(&self, operands: &[I]) -> Self {
        Self::new(self.symbol(), operands)
    }
}

impl<'a, B, I> PropositionalLogicBuilder<I> for Builder<'a, B, 2>
where
    I: Indexing,
    B: Buildable<2, IDX = I, Fragment = PropositionalLogic<I>>,
{
    #[inline(always)]
    fn var(&mut self, id: I) -> I {
        self.push(PropositionalLogic::Variable { id: id }, &[])
    }

    #[inline(always)]
    fn not<F: Fn(&mut Self) -> I>(&mut self, inner: F) -> I {
        let inner_id = inner(self);
        self.push(PropositionalLogic::Not, &[inner_id])
    }

    #[inline(always)]
    fn and<F: Fn(&mut Self) -> I, G: Fn(&mut Self) -> I>(&mut self, left: F, right: G) -> I {
        let left_id = left(self);
        let right_id = right(self);
        self.push(PropositionalLogic::And, &[left_id, right_id])
    }

    #[inline(always)]
    fn or<F: Fn(&mut Self) -> I, G: Fn(&mut Self) -> I>(&mut self, left: F, right: G) -> I {
        let left_id = left(self);
        let right_id = right(self);
        self.push(PropositionalLogic::Or, &[left_id, right_id])
    }
}
