use std::ops::Index;

use crate::tree::{
    builder::Builder,
    index::Indexing,
    node::{LinkinNode, Node},
    traits::{Buildable, Mapping},
};

use super::fragment::{Fragment, FragmentNode};

#[derive(Clone, Copy, Debug)]
pub enum ProbabilisticCircuit<IDX: Indexing = u32> {
    Variable { id: IDX, neg: bool },
    Product,
    Sum,
    Weight { w: f32 },
    None,
}

impl<I: Indexing> Fragment<I, 2> for ProbabilisticCircuit<I> {
    type Node = ProbabilisticNode<I>;
}

impl<IDX: Indexing> Default for ProbabilisticCircuit<IDX> {
    fn default() -> Self {
        ProbabilisticCircuit::None
    }
}

pub trait ProbabilisticCircuitBuilder<I: Indexing> {
    fn var(&mut self, id: I) -> I;
    fn not<F: Fn(&mut Self) -> I>(&mut self, inner: F) -> I;
    fn product<F: Fn(&mut Self) -> I, G: Fn(&mut Self) -> I>(&mut self, left: F, right: G) -> I;
    fn sum<F: Fn(&mut Self) -> I, G: Fn(&mut Self) -> I>(&mut self, left: F, right: G) -> I;
    fn weight<F: Fn(&mut Self) -> I>(&mut self, inner: F, w: f32) -> I;
}

pub type ProbabilisticNode<I> = Node<I, ProbabilisticCircuit<I>, 2>;

impl<I: Indexing> FragmentNode<I, ProbabilisticCircuit<I>, 2> for ProbabilisticNode<I> {
    fn fmt_display<T: Mapping<I> + Index<I, Output = Self>>(
        &self,
        f: &mut std::fmt::Formatter,
        tree: &T,
    ) -> std::fmt::Result {
        match self.symbol() {
            ProbabilisticCircuit::Variable { id, neg } => {
                let sign = if neg { "\u{00AC}" } else { "" };

                match tree.get_named(id) {
                    Some(name) => write!(f, "{}{}", sign, name),
                    None => write!(f, "{}X{}", sign, id.addr()),
                }
            }
            ProbabilisticCircuit::Product => {
                let mut operands = self.operands();
                write!(f, "(")?;
                tree[operands.next().unwrap()].fmt_display(f, tree)?;
                write!(f, "*")?;
                tree[operands.next().unwrap()].fmt_display(f, tree)?;
                write!(f, ")")
            }
            ProbabilisticCircuit::Sum => {
                let mut operands = self.operands();
                write!(f, "(")?;
                tree[operands.next().unwrap()].fmt_display(f, tree)?;
                write!(f, "+")?;
                tree[operands.next().unwrap()].fmt_display(f, tree)?;
                write!(f, ")")
            }
            ProbabilisticCircuit::Weight { w } => {
                let mut operands = self.operands();
                write!(f, "{:.3}(", w)?;
                tree[operands.next().unwrap()].fmt_display(f, tree)?;
                write!(f, ")")
            }
            ProbabilisticCircuit::None => panic!("Can't display a partially initialised tree."),
        }
    }

    fn arity(&self) -> usize {
        match self.symbol() {
            ProbabilisticCircuit::Variable { id: _, neg: _ } => 0,
            ProbabilisticCircuit::Product => 2,
            ProbabilisticCircuit::Sum => 2,
            ProbabilisticCircuit::Weight { w: _ } => 1,
            ProbabilisticCircuit::None => 0,
        }
    }

    fn new(symbol: ProbabilisticCircuit<I>, operands: &[I]) -> Self {
        Self::new(symbol, operands)
    }

    fn duplicate(&self, operands: &[I]) -> Self {
        Self::new(self.symbol(), operands)
    }
}

impl<'a, B, I> ProbabilisticCircuitBuilder<I> for Builder<'a, B, 2>
where
    I: Indexing,
    B: Buildable<2, IDX = I, Fragment = ProbabilisticCircuit<I>>,
{
    #[inline(always)]
    fn var(&mut self, id: I) -> I {
        self.push(ProbabilisticCircuit::Variable { id: id, neg: false }, &[])
    }

    #[inline(always)]
    fn not<F: Fn(&mut Self) -> I>(&mut self, inner: F) -> I {
        let inner_id = inner(self);
        match self.index_mut(inner_id).symbol_mut() {
            ProbabilisticCircuit::Variable { id: _, neg } => *neg = !*neg,
            _ => panic!("Negation can be applied to variables only."),
        }
        inner_id
    }

    #[inline(always)]
    fn product<F: Fn(&mut Self) -> I, G: Fn(&mut Self) -> I>(&mut self, left: F, right: G) -> I {
        let left_id = left(self);
        let right_id = right(self);
        self.push(ProbabilisticCircuit::Product, &[left_id, right_id])
    }

    #[inline(always)]
    fn sum<F: Fn(&mut Self) -> I, G: Fn(&mut Self) -> I>(&mut self, left: F, right: G) -> I {
        let left_id = left(self);
        let right_id = right(self);
        self.push(ProbabilisticCircuit::Sum, &[left_id, right_id])
    }

    #[inline(always)]
    fn weight<F: Fn(&mut Self) -> I>(&mut self, inner: F, w: f32) -> I {
        let inner_id = inner(self);
        self.push(ProbabilisticCircuit::Weight { w: w }, &[inner_id])
    }
}
