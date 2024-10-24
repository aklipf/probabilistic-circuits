use std::ops::IndexMut;

use crate::tree::{Addr, IndexedMutRef, IntoAddr, Node, NodeAllocator, NodeValue};

use super::FOLogic;

pub trait FOMut: Sized {
    fn pred<T: IntoAddr<Self, Addr>>(&mut self, id: T, vars_id: &[T]) -> Addr;
    fn every<T: IntoAddr<Self, Addr>, F: Fn(&mut Self) -> Addr>(&mut self, id: T, inner: F)
        -> Addr;
    fn exist<T: IntoAddr<Self, Addr>, F: Fn(&mut Self) -> Addr>(&mut self, id: T, inner: F)
        -> Addr;
    fn not<F: Fn(&mut Self) -> Addr>(&mut self, inner: F) -> Addr;
    fn and<F: Fn(&mut Self) -> Addr, G: Fn(&mut Self) -> Addr>(
        &mut self,
        left: F,
        right: G,
    ) -> Addr;
    fn or<F: Fn(&mut Self) -> Addr, G: Fn(&mut Self) -> Addr>(&mut self, left: F, right: G)
        -> Addr;
}

impl<'a, T> FOMut for IndexedMutRef<'a, T>
where
    T: IndexMut<Addr, Output = NodeValue<Node<2>, FOLogic>>
        + NodeAllocator<Value = FOLogic, Node = Node<2>>,
{
    #[inline(always)]
    fn pred<U: IntoAddr<Self, Addr>>(&mut self, id: U, vars_id: &[U]) -> Addr {
        let mut next_id = Addr::NONE;
        for &var_id in vars_id.iter().rev().chain(&[id]) {
            let addr = var_id.get_addr(self);
            next_id = self.array.push(FOLogic::Predicate { id: addr }, &[next_id])
        }
        next_id
    }

    #[inline(always)]
    fn every<U: IntoAddr<Self, Addr>, F: Fn(&mut Self) -> Addr>(
        &mut self,
        id: U,
        inner: F,
    ) -> Addr {
        let inner_id = inner(self);
        let addr = id.get_addr(self);
        self.array
            .push(FOLogic::Universal { id: addr }, &[inner_id])
    }

    #[inline(always)]
    fn exist<U: IntoAddr<Self, Addr>, F: Fn(&mut Self) -> Addr>(
        &mut self,
        id: U,
        inner: F,
    ) -> Addr {
        let inner_id = inner(self);
        let addr = id.get_addr(self);
        self.array
            .push(FOLogic::Existential { id: addr }, &[inner_id])
    }

    #[inline(always)]
    fn not<F: Fn(&mut Self) -> Addr>(&mut self, inner: F) -> Addr {
        let inner_id = inner(self);
        self.array.push(FOLogic::Not, &[inner_id])
    }

    #[inline(always)]
    fn and<F: Fn(&mut Self) -> Addr, G: Fn(&mut Self) -> Addr>(
        &mut self,
        left: F,
        right: G,
    ) -> Addr {
        let left_id = left(self);
        let right_id = right(self);
        self.array.push(FOLogic::And, &[left_id, right_id])
    }

    #[inline(always)]
    fn or<F: Fn(&mut Self) -> Addr, G: Fn(&mut Self) -> Addr>(
        &mut self,
        left: F,
        right: G,
    ) -> Addr {
        let left_id = left(self);
        let right_id = right(self);
        self.array.push(FOLogic::Or, &[left_id, right_id])
    }
}
