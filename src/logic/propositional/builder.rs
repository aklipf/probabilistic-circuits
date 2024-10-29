use std::ops::IndexMut;

use crate::tree::{Addr, IndexedMutRef, IntoAddr, Mapping, Node, NodeAllocator, NodeValue};

use super::PLogic;

pub trait PMut: Sized {
    fn var<T: IntoAddr<Self, Addr>>(&mut self, id: T) -> Addr;
    fn not<F: Fn(&mut Self) -> Addr>(&mut self, inner: F) -> Addr;
    fn and<F: Fn(&mut Self) -> Addr, G: Fn(&mut Self) -> Addr>(
        &mut self,
        left: F,
        right: G,
    ) -> Addr;
    fn or<F: Fn(&mut Self) -> Addr, G: Fn(&mut Self) -> Addr>(&mut self, left: F, right: G)
        -> Addr;
    fn conjunction<F: Fn(&mut Self, U::Item) -> Addr, U: Iterator>(
        &mut self,
        iter: &mut U,
        inner: F,
    ) -> Addr;
    fn disjunction<F: Fn(&mut Self, U::Item) -> Addr, U: Iterator>(
        &mut self,
        iter: &mut U,
        inner: F,
    ) -> Addr;
}

impl<'a, T> PMut for IndexedMutRef<'a, T>
where
    T: IndexMut<Addr, Output = NodeValue<Node<2>, PLogic>>
        + NodeAllocator<Value = PLogic, Node = Node<2>>
        + Mapping,
{
    #[inline(always)]
    fn var<U: IntoAddr<Self, Addr>>(&mut self, id: U) -> Addr {
        let addr = id.get_addr(self);
        self.array.push(PLogic::Variable { id: addr }, &[])
    }

    fn not<F: Fn(&mut Self) -> Addr>(&mut self, inner: F) -> Addr {
        let inner_id = inner(self);
        self.array.push(PLogic::Not, &[inner_id])
    }

    fn and<F: Fn(&mut Self) -> Addr, G: Fn(&mut Self) -> Addr>(
        &mut self,
        left: F,
        right: G,
    ) -> Addr {
        let left_id = left(self);
        let right_id = right(self);
        self.array.push(PLogic::And, &[left_id, right_id])
    }

    fn or<F: Fn(&mut Self) -> Addr, G: Fn(&mut Self) -> Addr>(
        &mut self,
        left: F,
        right: G,
    ) -> Addr {
        let left_id = left(self);
        let right_id = right(self);
        self.array.push(PLogic::Or, &[left_id, right_id])
    }

    fn conjunction<F: Fn(&mut Self, U::Item) -> Addr, U: Iterator>(
        &mut self,
        iter: &mut U,
        inner: F,
    ) -> Addr {
        let mut current_id = match iter.next() {
            Some(value) => inner(self, value),
            None => {
                return Addr::NONE;
            }
        };
        for next in iter {
            let inner_id = inner(self, next);
            current_id = self.array.push(PLogic::And, &[current_id, inner_id]);
        }
        current_id
    }

    fn disjunction<F: Fn(&mut Self, U::Item) -> Addr, U: Iterator>(
        &mut self,
        iter: &mut U,
        inner: F,
    ) -> Addr {
        let mut current_id = match iter.next() {
            Some(value) => inner(self, value),
            None => {
                return Addr::NONE;
            }
        };
        for next in iter {
            let inner_id = inner(self, next);
            current_id = self.array.push(PLogic::Or, &[current_id, inner_id]);
        }
        current_id
    }
}
