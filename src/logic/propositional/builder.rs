use std::ops::IndexMut;

use crate::tree::{
    Addr, IndexedMutRef, IntoAddr, LinkingNode, Mapping, Node, NodeAllocator, NodeValue,
};

use super::{PLogic, PRef};

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

    fn clone<T: PRef>(&mut self, node: &T) -> Addr;
    fn clone_id(&mut self, id: Addr) -> Addr;

    fn into_var<T: IntoAddr<Self, Addr>>(&mut self, id: T);
    fn into_not<F: Fn(&mut Self) -> Addr>(&mut self, inner: F);
    fn into_and<F: Fn(&mut Self) -> Addr, G: Fn(&mut Self) -> Addr>(&mut self, left: F, right: G);
    fn into_or<F: Fn(&mut Self) -> Addr, G: Fn(&mut Self) -> Addr>(&mut self, left: F, right: G);
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
        let parent_id = self.array.push(PLogic::Not, &[inner_id]);

        self.array[inner_id].node.replace_parent(parent_id);

        parent_id
    }

    fn and<F: Fn(&mut Self) -> Addr, G: Fn(&mut Self) -> Addr>(
        &mut self,
        left: F,
        right: G,
    ) -> Addr {
        let left_id = left(self);
        let right_id = right(self);
        let parent_id = self.array.push(PLogic::And, &[left_id, right_id]);

        self.array[left_id].node.replace_parent(parent_id);
        self.array[right_id].node.replace_parent(parent_id);

        parent_id
    }

    fn or<F: Fn(&mut Self) -> Addr, G: Fn(&mut Self) -> Addr>(
        &mut self,
        left: F,
        right: G,
    ) -> Addr {
        let left_id = left(self);
        let right_id = right(self);
        let parent_id = self.array.push(PLogic::Or, &[left_id, right_id]);

        self.array[left_id].node.replace_parent(parent_id);
        self.array[right_id].node.replace_parent(parent_id);

        parent_id
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
            let and_id = self.array.push(PLogic::And, &[current_id, inner_id]);

            self.array[current_id].node.replace_parent(and_id);
            self.array[inner_id].node.replace_parent(and_id);
            current_id = and_id;
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
            let or_id = self.array.push(PLogic::Or, &[current_id, inner_id]);

            self.array[current_id].node.replace_parent(or_id);
            self.array[inner_id].node.replace_parent(or_id);
            current_id = or_id;
        }
        current_id
    }

    fn into_var<U: IntoAddr<Self, Addr>>(&mut self, id: U) {
        let addr = id.get_addr(self);

        self.array[self.idx].value = PLogic::Variable { id: addr };
        self.array[self.idx].node.remove_operands();
    }

    fn into_not<F: Fn(&mut Self) -> Addr>(&mut self, inner: F) {
        let inner_id = inner(self);
        IndexedMutRef {
            array: self.array,
            idx: inner_id,
        }
        .as_mut()
        .node
        .replace_parent(self.idx);

        let node = &mut self.array[self.idx];
        node.value = PLogic::Not;
        node.node.replace_operands(&[inner_id]);
    }

    fn into_and<F: Fn(&mut Self) -> Addr, G: Fn(&mut Self) -> Addr>(&mut self, left: F, right: G) {
        let left_id = left(self);
        IndexedMutRef {
            array: self.array,
            idx: left_id,
        }
        .as_mut()
        .node
        .replace_parent(self.idx);

        let right_id = right(self);
        IndexedMutRef {
            array: self.array,
            idx: right_id,
        }
        .as_mut()
        .node
        .replace_parent(self.idx);

        let node = &mut self.array[self.idx];
        node.value = PLogic::And;
        node.node.replace_operands(&[left_id, right_id]);
    }

    fn into_or<F: Fn(&mut Self) -> Addr, G: Fn(&mut Self) -> Addr>(&mut self, left: F, right: G) {
        let left_id = left(self);
        IndexedMutRef {
            array: self.array,
            idx: left_id,
        }
        .as_mut()
        .node
        .replace_parent(self.idx);

        let right_id = right(self);
        IndexedMutRef {
            array: self.array,
            idx: right_id,
        }
        .as_mut()
        .node
        .replace_parent(self.idx);

        let node = &mut self.array[self.idx];
        node.value = PLogic::Or;
        node.node.replace_operands(&[left_id, right_id]);
    }

    fn clone<U: PRef>(&mut self, node: &U) -> Addr {
        match self.as_ref().value {
            PLogic::Variable { id } => self.var(id),
            PLogic::Not => self.not(|inner| inner.clone(&node.inner())),
            PLogic::And => self.and(
                |left| left.clone(&node.left()),
                |right| right.clone(&node.right()),
            ),
            PLogic::Or => self.or(
                |left| left.clone(&node.left()),
                |right| right.clone(&node.right()),
            ),
        }
    }

    fn clone_id(&mut self, id: Addr) -> Addr {
        if let &[left_id, right_id] = self.array[id].node.operands() {
            match self.array[id].value {
                PLogic::Variable { id } => self.var(id),
                PLogic::Not => self.not(|inner| inner.clone_id(left_id)),
                PLogic::And => self.and(
                    |left| left.clone_id(left_id),
                    |right| right.clone_id(right_id),
                ),
                PLogic::Or => self.or(
                    |left| left.clone_id(left_id),
                    |right| right.clone_id(right_id),
                ),
            }
        } else {
            panic!()
        }
    }
}
