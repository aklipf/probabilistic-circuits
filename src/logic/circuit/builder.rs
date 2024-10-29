use std::ops::IndexMut;

use crate::tree::{Addr, IndexedMutRef, IntoAddr, Node, NodeAllocator, NodeValue};

use super::PCicruit;

pub trait PCMut: Sized {
    fn var<T: IntoAddr<Self, Addr>>(&mut self, id: T) -> Addr;
    fn not_var<T: IntoAddr<Self, Addr>>(&mut self, id: T) -> Addr;
    fn prod<F: Fn(&mut Self) -> Addr, G: Fn(&mut Self) -> Addr>(
        &mut self,
        left: F,
        right: G,
    ) -> Addr;
    fn sum<F: Fn(&mut Self) -> Addr, G: Fn(&mut Self) -> Addr>(
        &mut self,
        left: F,
        right: G,
    ) -> Addr;
    fn sum_w<F: Fn(&mut Self) -> Addr, G: Fn(&mut Self) -> Addr>(
        &mut self,
        weight_left: f32,
        left: F,
        weight_right: f32,
        right: G,
    ) -> Addr;
    fn prod_n<F: Fn(&mut Self, T::Item) -> Addr, T: Iterator>(
        &mut self,
        iter: &mut T,
        inner: F,
    ) -> Addr;
    fn sum_n<F: Fn(&mut Self, T::Item) -> (Addr, f32), T: Iterator>(
        &mut self,
        iter: &mut T,
        inner: F,
    ) -> Addr;
}

impl<'a, T> PCMut for IndexedMutRef<'a, T>
where
    T: IndexMut<Addr, Output = NodeValue<Node<2>, PCicruit>>
        + NodeAllocator<Value = PCicruit, Node = Node<2>>,
{
    #[inline(always)]
    fn var<U: IntoAddr<Self, Addr>>(&mut self, id: U) -> Addr {
        let addr = id.get_addr(self);
        self.array.push(
            PCicruit::Variable {
                id: addr,
                neg: false,
            },
            &[],
        )
    }

    #[inline(always)]
    fn not_var<U: IntoAddr<Self, Addr>>(&mut self, id: U) -> Addr {
        let addr = id.get_addr(self);
        self.array.push(
            PCicruit::Variable {
                id: addr,
                neg: true,
            },
            &[],
        )
    }

    #[inline(always)]
    fn prod<F: Fn(&mut Self) -> Addr, G: Fn(&mut Self) -> Addr>(
        &mut self,
        left: F,
        right: G,
    ) -> Addr {
        let left_id = left(self);
        let right_id = right(self);
        self.array.push(PCicruit::Product, &[left_id, right_id])
    }

    #[inline(always)]
    fn sum<F: Fn(&mut Self) -> Addr, G: Fn(&mut Self) -> Addr>(
        &mut self,
        left: F,
        right: G,
    ) -> Addr {
        let left_id = left(self);
        let right_id = right(self);
        self.array.push(
            PCicruit::Sum {
                left: 1.0,
                right: 1.0,
            },
            &[left_id, right_id],
        )
    }

    #[inline(always)]
    fn sum_w<F: Fn(&mut Self) -> Addr, G: Fn(&mut Self) -> Addr>(
        &mut self,
        weight_left: f32,
        left: F,
        weight_right: f32,
        right: G,
    ) -> Addr {
        let left_id = left(self);
        let right_id = right(self);
        self.array.push(
            PCicruit::Sum {
                left: weight_left,
                right: weight_right,
            },
            &[left_id, right_id],
        )
    }

    fn prod_n<F: Fn(&mut Self, U::Item) -> Addr, U: Iterator>(
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
            current_id = self.array.push(PCicruit::Product, &[current_id, inner_id]);
        }
        current_id
    }

    fn sum_n<F: Fn(&mut Self, U::Item) -> (Addr, f32), U: Iterator>(
        &mut self,
        iter: &mut U,
        inner: F,
    ) -> Addr {
        let (mut current_id, mut current_w) = match iter.next() {
            Some(value) => inner(self, value),
            None => {
                return Addr::NONE;
            }
        };
        for next in iter {
            let (inner_id, inner_weight) = inner(self, next);
            current_id = self.array.push(
                PCicruit::Sum {
                    left: current_w,
                    right: inner_weight,
                },
                &[current_id, inner_id],
            );
            current_w = 1.0;
        }
        current_id
    }
}
