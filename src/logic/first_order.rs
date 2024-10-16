use std::ops::Index;

use crate::tree::{
    builder::Builder,
    index::Indexing,
    node::{LinkinNode, Node},
    traits::{Buildable, Mapping},
};

use super::fragment::{Fragment, FragmentNode};

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

pub trait FirstOrderLogicBuilder<I: Indexing> {
    fn pred(&mut self, id: I, vars_id: &[I]) -> I;
    fn every<F: Fn(&mut Self) -> I>(&mut self, id: I, inner: F) -> I;
    fn exist<F: Fn(&mut Self) -> I>(&mut self, id: I, inner: F) -> I;
    fn not<F: Fn(&mut Self) -> I>(&mut self, inner: F) -> I;
    fn and<F: Fn(&mut Self) -> I, G: Fn(&mut Self) -> I>(&mut self, left: F, right: G) -> I;
    fn or<F: Fn(&mut Self) -> I, G: Fn(&mut Self) -> I>(&mut self, left: F, right: G) -> I;
}

pub type FirstOrderNode<I> = Node<I, FirstOrderLogic<I>, 2>;

impl<I: Indexing> FragmentNode<I, FirstOrderLogic<I>, 2> for FirstOrderNode<I> {
    fn fmt_display<T: Mapping<I> + Index<I, Output = Self>>(
        &self,
        f: &mut std::fmt::Formatter,
        tree: &T,
    ) -> std::fmt::Result {
        match self.symbol() {
            FirstOrderLogic::Predicate { id } => {
                match tree.get_named(id) {
                    Some(pred_name) => write!(f, "{}(", pred_name),
                    None => write!(f, "Anon{}(", id.addr()),
                }?;

                let mut first_var = true;
                let mut next_id = self.operands().next();
                if next_id.is_none() {
                    return write!(f, ")");
                }

                while let Some(node_id) = next_id {
                    if first_var {
                        first_var = false;
                    } else {
                        write!(f, ", ")?;
                    }

                    match tree[node_id].symbol() {
                        FirstOrderLogic::Predicate { id: var_id } => match tree.get_named(var_id) {
                            Some(name) => write!(f, "{}", name),
                            None => write!(f, "Anon{}", var_id.addr()),
                        },
                        _ => panic!(),
                    }?;

                    next_id = tree[node_id].operands().next();
                }
                write!(f, ")")
            }
            FirstOrderLogic::Not => {
                write!(f, "\u{00AC}")?;
                tree[self.operands().next().unwrap()].fmt_display(f, tree)
            }
            FirstOrderLogic::And => {
                let mut operands = self.operands();
                write!(f, "(")?;
                tree[operands.next().unwrap()].fmt_display(f, tree)?;
                write!(f, "\u{2227}")?;
                tree[operands.next().unwrap()].fmt_display(f, tree)?;
                write!(f, ")")
            }
            FirstOrderLogic::Or => {
                let mut operands = self.operands();
                write!(f, "(")?;
                tree[operands.next().unwrap()].fmt_display(f, tree)?;
                write!(f, "\u{2228}")?;
                tree[operands.next().unwrap()].fmt_display(f, tree)?;
                write!(f, ")")
            }
            FirstOrderLogic::Universal { id } => {
                write!(
                    f,
                    "\u{2200}{}:",
                    tree.get_named(id).unwrap_or(&format!("Anon{}", id.addr()))
                )?;
                tree[self.operands().next().unwrap()].fmt_display(f, tree)
            }
            FirstOrderLogic::Existential { id } => {
                write!(
                    f,
                    "\u{2203}{}:",
                    tree.get_named(id).unwrap_or(&format!("Anon{}", id.addr()))
                )?;
                tree[self.operands().next().unwrap()].fmt_display(f, tree)
            }
            FirstOrderLogic::None => panic!("Can't display a partially initialised tree."),
        }
    }

    fn arity(&self) -> usize {
        match self.symbol() {
            FirstOrderLogic::Predicate { id: _ } => 0,
            FirstOrderLogic::Universal { id: _ } => 1,
            FirstOrderLogic::Existential { id: _ } => 1,
            FirstOrderLogic::Not => 1,
            FirstOrderLogic::And => 2,
            FirstOrderLogic::Or => 2,
            FirstOrderLogic::None => 0,
        }
    }

    fn new(symbol: FirstOrderLogic<I>, operands: &[I]) -> Self {
        Self::new(symbol, operands)
    }

    fn duplicate(&self, operands: &[I]) -> Self {
        Self::new(self.symbol(), operands)
    }
}

impl<'a, B, I> FirstOrderLogicBuilder<I> for Builder<'a, B, 2>
where
    I: Indexing,
    B: Buildable<2, IDX = I, Fragment = FirstOrderLogic<I>>,
{
    #[inline(always)]
    fn pred(&mut self, id: I, vars_id: &[I]) -> I {
        let mut next_id = I::NONE;
        for &var_id in vars_id.iter().rev() {
            next_id = self.push(FirstOrderLogic::Predicate { id: var_id }, &[next_id])
        }

        self.push(FirstOrderLogic::Predicate { id: id }, &[next_id])
    }

    #[inline(always)]
    fn every<F: Fn(&mut Self) -> I>(&mut self, id: I, inner: F) -> I {
        let inner_id = inner(self);
        self.push(FirstOrderLogic::Universal { id: id }, &[inner_id])
    }

    #[inline(always)]
    fn exist<F: Fn(&mut Self) -> I>(&mut self, id: I, inner: F) -> I {
        let inner_id = inner(self);
        self.push(FirstOrderLogic::Existential { id: id }, &[inner_id])
    }

    #[inline(always)]
    fn not<F: Fn(&mut Self) -> I>(&mut self, inner: F) -> I {
        let inner_id = inner(self);
        self.push(FirstOrderLogic::Not, &[inner_id])
    }

    #[inline(always)]
    fn and<F: Fn(&mut Self) -> I, G: Fn(&mut Self) -> I>(&mut self, left: F, right: G) -> I {
        let left_id = left(self);
        let right_id = right(self);
        self.push(FirstOrderLogic::And, &[left_id, right_id])
    }

    #[inline(always)]
    fn or<F: Fn(&mut Self) -> I, G: Fn(&mut Self) -> I>(&mut self, left: F, right: G) -> I {
        let left_id = left(self);
        let right_id = right(self);
        self.push(FirstOrderLogic::Or, &[left_id, right_id])
    }
}
