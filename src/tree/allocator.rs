use std::ops::{Index, IndexMut};

use crate::logic::fragment::{Fragment, Symbols};

use super::mapping::Mapping;
use super::tree::Tree;

use super::{index::Indexing, node::Node};

pub trait Allocator
where
    Self::IDX: Indexing,
    Self::Symbols: Symbols,
{
    type Symbols;
    type IDX;
    fn push(&mut self, symbol: Self::Symbols, operands: &[Self::IDX]) -> Self::IDX;
}

impl<S: Symbols, IDX: Indexing, const MAX_CHILDS: usize> Allocator for Tree<S, IDX, MAX_CHILDS>
where
    Node<IDX, S, MAX_CHILDS>: Fragment<IDX>,
{
    type Symbols = S;
    type IDX = IDX;

    fn push(&mut self, symbol: S, operands: &[Self::IDX]) -> Self::IDX {
        let idx = IDX::from(self.nodes.len());
        let mut childs = [IDX::NONE; MAX_CHILDS];

        childs
            .iter_mut()
            .zip(operands)
            .for_each(|(dst, src)| *dst = *src);

        self.nodes.push(Node {
            parent: IDX::NONE,
            childs: childs,
            symbol: symbol,
        });
        idx
    }
}

#[macro_export]
macro_rules! recycle {
    ($root:expr) => {
        |recycler| recycler.cut($root, &[])
    };
    ($root:expr,$($leafs:expr),*) => {
        |recycler| recycler.cut($root, &[$($leafs),*])
    };
}

pub struct Recycle<'a, S: Symbols, IDX: Indexing, const MAX_CHILDS: usize>
where
    Node<IDX, S, MAX_CHILDS>: Fragment<IDX>,
{
    pub(super) tree: &'a mut Tree<S, IDX, MAX_CHILDS>,
    pub(super) root: IDX,
    pub(super) current_idx: IDX,
}

impl<'a, S: Symbols, IDX: Indexing, const MAX_CHILDS: usize> Recycle<'a, S, IDX, MAX_CHILDS>
where
    Node<IDX, S, MAX_CHILDS>: Fragment<IDX>,
{
    pub fn new(tree: &'a mut Tree<S, IDX, MAX_CHILDS>) -> Self {
        Recycle {
            tree: tree,
            root: IDX::NONE,
            current_idx: IDX::NONE,
        }
    }

    pub fn cut(&mut self, from_node: IDX, until_nodes: &[IDX]) {
        self.root = self.tree[from_node].parent;
        if self.root.is_addr() {
            let _ = self.tree[self.root].replace_operand(from_node, IDX::NONE);
        }
        self.tree[from_node].parent.unlink();

        for &idx in until_nodes {
            let parent = self.tree[idx].parent;
            self.tree[parent].childs[0].unlink();
            self.tree[parent].childs[1].unlink();
        }
    }

    fn next(&mut self) -> Option<IDX> {
        let current_idx = self.current_idx;

        if current_idx.is_none() {
            return None;
        }

        let node = &mut self.tree[self.current_idx];

        match node.childs.iter_mut().find(|idx| idx.is_addr()) {
            Some(idx) => {
                self.current_idx = *idx;
                idx.unlink();
                self.next()
            }
            None => {
                self.current_idx = node.parent;
                node.parent.unlink();
                Some(current_idx)
            }
        }
    }

    fn replace_with_last(&mut self, idx: IDX) {
        let initial_idx = IDX::from(self.tree.nodes.len() - 1);

        // replace iterator position if needed
        if initial_idx == self.current_idx {
            self.current_idx = idx;
        }

        // pop last node
        let last_node = self
            .tree
            .nodes
            .pop()
            .expect("Cannot replace node in empty tree");

        // copy reconnect last node if needed
        if idx.addr() < self.tree.nodes.len() {
            for &child_idx in last_node.childs_idx() {
                self.tree[child_idx].parent = idx;
            }
            if last_node.parent.is_addr() {
                self.tree[last_node.parent]
                    .replace_operand(initial_idx, idx)
                    .expect("Recycle error");
            }

            self.tree[idx] = last_node;
        }
    }
}

impl<'a, S: Symbols, IDX: Indexing, const MAX_CHILDS: usize> Index<IDX>
    for Recycle<'a, S, IDX, MAX_CHILDS>
where
    Node<IDX, S, MAX_CHILDS>: Fragment<IDX>,
{
    type Output = Node<IDX, S, MAX_CHILDS>;

    #[inline]
    fn index(&self, index: IDX) -> &Self::Output {
        &self.tree[index]
    }
}

impl<'a, S: Symbols, IDX: Indexing, const MAX_CHILDS: usize> IndexMut<IDX>
    for Recycle<'a, S, IDX, MAX_CHILDS>
where
    Node<IDX, S, MAX_CHILDS>: Fragment<IDX>,
{
    #[inline]
    fn index_mut<'b>(&'b mut self, index: IDX) -> &'b mut Node<IDX, S, MAX_CHILDS> {
        &mut self.tree[index]
    }
}

impl<'a, S: Symbols, IDX: Indexing, const MAX_CHILDS: usize> Allocator
    for Recycle<'a, S, IDX, MAX_CHILDS>
where
    Node<IDX, S, MAX_CHILDS>: Fragment<IDX>,
{
    type Symbols = S;
    type IDX = IDX;

    fn push(&mut self, symbol: S, operands: &[Self::IDX]) -> Self::IDX {
        match self.next() {
            Some(idx) => {
                let node = &mut self.tree[idx];
                node.symbol = symbol;
                node.childs = [IDX::NONE; MAX_CHILDS];
                node.childs
                    .iter_mut()
                    .zip(operands)
                    .for_each(|(dst, src)| *dst = *src);

                idx
            }
            None => {
                let idx = IDX::from(self.tree.nodes.len());

                let mut childs = [IDX::NONE; MAX_CHILDS];

                childs
                    .iter_mut()
                    .zip(operands)
                    .for_each(|(dst, src)| *dst = *src);

                self.tree.nodes.push(Node {
                    parent: IDX::NONE,
                    childs: childs,
                    symbol: symbol,
                });

                idx
            }
        }
    }
}

impl<'a, S: Symbols, IDX: Indexing, const MAX_CHILDS: usize> Drop
    for Recycle<'a, S, IDX, MAX_CHILDS>
where
    Node<IDX, S, MAX_CHILDS>: Fragment<IDX>,
{
    fn drop(&mut self) {
        while let Some(idx) = self.next() {
            self.replace_with_last(idx)
        }
    }
}

impl<'a, S: Symbols, IDX: Indexing, const MAX_CHILDS: usize> Mapping
    for Recycle<'a, S, IDX, MAX_CHILDS>
where
    Node<IDX, S, MAX_CHILDS>: Fragment<IDX>,
{
    type IDX = IDX;

    fn add_named(&mut self, name: &String) -> IDX {
        self.tree.add_named(name)
    }

    fn add_anon(&mut self) -> IDX {
        self.tree.add_anon()
    }

    fn get_id(&self, name: &String) -> Option<IDX> {
        self.tree.get_id(name)
    }

    fn get_named(&self, id: IDX) -> Option<&String> {
        self.tree.get_named(id)
    }
}
