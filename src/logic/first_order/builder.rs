use crate::tree::{builder::Builder, index::Indexing, traits::Buildable};

use super::FirstOrderLogic;

pub trait FirstOrderLogicBuilder<I: Indexing> {
    fn pred(&mut self, id: I, vars_id: &[I]) -> I;
    fn every<F: Fn(&mut Self) -> I>(&mut self, id: I, inner: F) -> I;
    fn exist<F: Fn(&mut Self) -> I>(&mut self, id: I, inner: F) -> I;
    fn not<F: Fn(&mut Self) -> I>(&mut self, inner: F) -> I;
    fn and<F: Fn(&mut Self) -> I, G: Fn(&mut Self) -> I>(&mut self, left: F, right: G) -> I;
    fn or<F: Fn(&mut Self) -> I, G: Fn(&mut Self) -> I>(&mut self, left: F, right: G) -> I;
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
