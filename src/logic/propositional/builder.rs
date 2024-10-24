use std::ops::IndexMut;

use crate::tree::{Addr, IndexedMutRef, Mapping, Node, NodeAllocator, NodeValue};

use super::PLogic;

pub trait PMut {
    fn var_id(&mut self, id: Addr) -> Addr;
    fn not<F: Fn(&mut Self) -> Addr>(&mut self, inner: F) -> Addr;
    fn and<F: Fn(&mut Self) -> Addr, G: Fn(&mut Self) -> Addr>(
        &mut self,
        left: F,
        right: G,
    ) -> Addr;
    fn or<F: Fn(&mut Self) -> Addr, G: Fn(&mut Self) -> Addr>(&mut self, left: F, right: G)
        -> Addr;
}

pub trait NamedVariable {
    fn var(&mut self, name: &str) -> Addr;
}

impl<'a, T> NamedVariable for IndexedMutRef<'a, T>
where
    T: IndexMut<Addr, Output = NodeValue<Node<2>, PLogic>>
        + NodeAllocator<Value = PLogic, Node = Node<2>>
        + Mapping,
{
    fn var(&mut self, name: &str) -> Addr {
        let name_id = self.array.add_named(&name.to_string());
        self.array.push(PLogic::Variable { id: name_id }, &[])
    }
}

impl<'a, T> PMut for IndexedMutRef<'a, T>
where
    T: IndexMut<Addr, Output = NodeValue<Node<2>, PLogic>>
        + NodeAllocator<Value = PLogic, Node = Node<2>>,
{
    #[inline(always)]
    fn var_id(&mut self, id: Addr) -> Addr {
        self.array.push(PLogic::Variable { id: id }, &[])
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
}
