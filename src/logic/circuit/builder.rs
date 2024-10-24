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
}
