use crate::tree::{builder::Builder, index::Indexing, traits::Buildable};

use super::PropositionalLogic;

pub trait PropositionalLogicBuilder<I: Indexing> {
    fn var(&mut self, id: I) -> I;
    fn not<F: Fn(&mut Self) -> I>(&mut self, inner: F) -> I;
    fn and<F: Fn(&mut Self) -> I, G: Fn(&mut Self) -> I>(&mut self, left: F, right: G) -> I;
    fn or<F: Fn(&mut Self) -> I, G: Fn(&mut Self) -> I>(&mut self, left: F, right: G) -> I;
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
