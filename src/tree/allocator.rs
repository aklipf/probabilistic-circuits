use std::ops::{Index, IndexMut};

use crate::logic::fragment::{Fragment, FragmentNode};

use super::builder::Buildable;
use super::mapping::Mapping;
use super::node::LinkinNode;
use super::tree::Tree;

use super::index::Indexing;

pub trait Allocator<const MAX_CHILDS: usize>
where
    Self::IDX: Indexing,
    Self::Fragment: Fragment<Self::IDX, MAX_CHILDS>,
{
    type Fragment;
    type IDX;
    fn push(&mut self, symbol: Self::Fragment, operands: &[Self::IDX]) -> Self::IDX;
    fn push_node(
        &mut self,
        node: &<Self::Fragment as Fragment<Self::IDX, MAX_CHILDS>>::Node,
        operands: &[Self::IDX],
    ) -> Self::IDX;
}

pub trait Remover<I: Indexing> {
    fn remove(&mut self, idx: I) -> Result<I, &'static str>;
}

pub trait Removable<const MAX_CHILDS: usize>:
    Buildable<MAX_CHILDS> + Remover<<Self as Buildable<MAX_CHILDS>>::IDX>
{
}

impl<F, I, const MAX_CHILDS: usize> Removable<MAX_CHILDS> for Tree<F, I, MAX_CHILDS>
where
    I: Indexing,
    F: Fragment<I, MAX_CHILDS>,
{
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

pub struct Recycle<'a, R, const MAX_CHILDS: usize>
where
    R: Removable<MAX_CHILDS>,
{
    remover: &'a mut R,
    root: <R as Buildable<MAX_CHILDS>>::IDX,
    current_idx: <R as Buildable<MAX_CHILDS>>::IDX,
}

impl<'a, R, const MAX_CHILDS: usize> Buildable<MAX_CHILDS> for Recycle<'a, R, MAX_CHILDS>
where
    R: Removable<MAX_CHILDS>,
{
    type Fragment = <R as Buildable<MAX_CHILDS>>::Fragment;
    type IDX = <R as Buildable<MAX_CHILDS>>::IDX;
}

impl<'a, R, const MAX_CHILDS: usize> Recycle<'a, R, MAX_CHILDS>
where
    R: Removable<MAX_CHILDS>,
{
    pub fn root(&self) -> <R as Buildable<MAX_CHILDS>>::IDX {
        self.root
    }

    pub fn new(tree: &'a mut R) -> Self {
        Recycle {
            remover: tree,
            root: <R as Buildable<MAX_CHILDS>>::IDX::NONE,
            current_idx: <R as Buildable<MAX_CHILDS>>::IDX::NONE,
        }
    }

    pub fn cut(
        &mut self,
        from_node: <R as Buildable<MAX_CHILDS>>::IDX,
        until_nodes: &[<R as Buildable<MAX_CHILDS>>::IDX],
    ) {
        self.root = self.remover[from_node].parent();
        if self.root.is_addr() {
            let _ = self.remover[self.root]
                .replace_operand(from_node, <R as Buildable<MAX_CHILDS>>::IDX::NONE);
        }
        self.remover[from_node].unlink_parent();

        for &idx in until_nodes {
            let parent = self.remover[idx].parent();
            self.remover[parent].remove_operands();
        }
    }

    fn next(&mut self) -> Option<<R as Buildable<MAX_CHILDS>>::IDX> {
        let current_idx = self.current_idx;

        if current_idx.is_none() {
            return None;
        }

        let node = &mut self.remover[self.current_idx];

        match node.pop_operand() {
            Some(idx) => {
                self.current_idx = idx;
                self.next()
            }
            None => {
                self.current_idx = node.parent();
                node.unlink_parent();
                Some(current_idx)
            }
        }
    }

    fn remove(&mut self, idx: <R as Buildable<MAX_CHILDS>>::IDX) {
        // replace iterator position if needed
        if self.remover.remove(idx).expect("Recycle error") == self.current_idx {
            self.current_idx = idx;
        }
    }
}

impl<'a, R, const MAX_CHILDS: usize> Drop for Recycle<'a, R, MAX_CHILDS>
where
    R: Removable<MAX_CHILDS>,
{
    fn drop(&mut self) {
        while let Some(idx) = self.next() {
            self.remove(idx)
        }
    }
}

impl<'a, R, const MAX_CHILDS: usize> Allocator<MAX_CHILDS> for Recycle<'a, R, MAX_CHILDS>
where
    R: Removable<MAX_CHILDS>,
{
    type Fragment = <R as Buildable<MAX_CHILDS>>::Fragment;
    type IDX = <R as Buildable<MAX_CHILDS>>::IDX;

    fn push(&mut self, symbol: Self::Fragment, operands: &[Self::IDX]) -> Self::IDX {
        match self.next() {
            Some(idx) => {
                self.remover[idx] = <Self::Fragment as Fragment<Self::IDX, MAX_CHILDS>>::Node::new(
                    symbol, operands,
                );
                idx
            }
            None => self.remover.push(symbol, operands),
        }
    }

    fn push_node(
        &mut self,
        node: &<Self::Fragment as Fragment<Self::IDX, MAX_CHILDS>>::Node,
        operands: &[Self::IDX],
    ) -> Self::IDX {
        match self.next() {
            Some(idx) => {
                self.remover[idx] = node.duplicate(operands);
                idx
            }
            None => self.remover.push_node(node, operands),
        }
    }
}

impl<'a, R, const MAX_CHILDS: usize> Index<<R as Buildable<MAX_CHILDS>>::IDX>
    for Recycle<'a, R, MAX_CHILDS>
where
    R: Removable<MAX_CHILDS>,
{
    type Output = <<R as Buildable<MAX_CHILDS>>::Fragment as Fragment<
        <R as Buildable<MAX_CHILDS>>::IDX,
        MAX_CHILDS,
    >>::Node;

    #[inline(always)]
    fn index(&self, index: <R as Buildable<MAX_CHILDS>>::IDX) -> &Self::Output {
        self.remover.index(index)
    }
}

impl<'a, R, const MAX_CHILDS: usize> IndexMut<<R as Buildable<MAX_CHILDS>>::IDX>
    for Recycle<'a, R, MAX_CHILDS>
where
    R: Removable<MAX_CHILDS>,
{
    #[inline(always)]
    fn index_mut(&mut self, index: <R as Buildable<MAX_CHILDS>>::IDX) -> &mut Self::Output {
        self.remover.index_mut(index)
    }
}

impl<'a, R, const MAX_CHILDS: usize> Mapping for Recycle<'a, R, MAX_CHILDS>
where
    R: Removable<MAX_CHILDS>,
{
    type IDX = <R as Buildable<MAX_CHILDS>>::IDX;

    #[inline(always)]
    fn add_named(&mut self, name: &String) -> Self::IDX {
        self.remover.add_named(name)
    }

    #[inline(always)]
    fn add_anon(&mut self) -> Self::IDX {
        self.remover.add_anon()
    }

    #[inline(always)]
    fn get_id(&self, name: &String) -> Option<Self::IDX> {
        self.remover.get_id(name)
    }

    #[inline(always)]
    fn get_named(&self, id: Self::IDX) -> Option<&String> {
        self.remover.get_named(id)
    }
}
