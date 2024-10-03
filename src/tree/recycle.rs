use std::ops::{Index, IndexMut};

use crate::logic::fragment::{Fragment, FragmentNode};

use super::index::Indexing;
use super::node::LinkinNode;
use super::traits::{Allocator, Buildable, Mapping, Removable};
use super::tree::Tree;

impl<F, I, const MAX_CHILDS: usize> Removable<MAX_CHILDS> for Tree<F, I, MAX_CHILDS>
where
    I: Indexing,
    F: Fragment<I, MAX_CHILDS>,
{
}
pub struct Recycle<'a, R, const MAX_CHILDS: usize>
where
    R: Removable<MAX_CHILDS>,
{
    remover: &'a mut R,
    root: R::IDX,
    current_idx: R::IDX,
}

impl<'a, R, const MAX_CHILDS: usize> Buildable<MAX_CHILDS> for Recycle<'a, R, MAX_CHILDS>
where
    R: Removable<MAX_CHILDS>,
{
    type Fragment = R::Fragment;
    type IDX = R::IDX;
}

impl<'a, R, const MAX_CHILDS: usize> Recycle<'a, R, MAX_CHILDS>
where
    R: Removable<MAX_CHILDS>,
{
    pub fn root(&self) -> R::IDX {
        self.root
    }

    pub fn new(tree: &'a mut R) -> Self {
        Recycle {
            remover: tree,
            root: R::IDX::NONE,
            current_idx: R::IDX::NONE,
        }
    }

    pub fn cut(&mut self, from_node: R::IDX, until_nodes: &[R::IDX]) {
        self.root = self.remover[from_node].parent();
        if self.root.is_addr() {
            let _ = self.remover[self.root].replace_operand(from_node, R::IDX::NONE);
        }
        self.remover[from_node].unlink_parent();

        for &idx in until_nodes {
            let parent = self.remover[idx].parent();
            self.remover[parent].remove_operands();
        }
    }

    fn next(&mut self) -> Option<R::IDX> {
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

    fn remove(&mut self, idx: R::IDX) {
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

impl<'a, R, const MAX_CHILDS: usize> Allocator<R::IDX, R::Fragment, MAX_CHILDS>
    for Recycle<'a, R, MAX_CHILDS>
where
    R: Removable<MAX_CHILDS>,
{
    fn push(&mut self, symbol: R::Fragment, operands: &[R::IDX]) -> R::IDX {
        match self.next() {
            Some(idx) => {
                self.remover[idx] =
                    <R::Fragment as Fragment<R::IDX, MAX_CHILDS>>::Node::new(symbol, operands);
                idx
            }
            None => self.remover.push(symbol, operands),
        }
    }

    fn push_node(
        &mut self,
        node: &<R::Fragment as Fragment<R::IDX, MAX_CHILDS>>::Node,
        operands: &[R::IDX],
    ) -> R::IDX {
        match self.next() {
            Some(idx) => {
                self.remover[idx] = node.duplicate(operands);
                idx
            }
            None => self.remover.push_node(node, operands),
        }
    }
}

impl<'a, R, const MAX_CHILDS: usize> Index<R::IDX> for Recycle<'a, R, MAX_CHILDS>
where
    R: Removable<MAX_CHILDS>,
{
    type Output = <R::Fragment as Fragment<R::IDX, MAX_CHILDS>>::Node;

    #[inline(always)]
    fn index(&self, index: R::IDX) -> &Self::Output {
        self.remover.index(index)
    }
}

impl<'a, R, const MAX_CHILDS: usize> IndexMut<R::IDX> for Recycle<'a, R, MAX_CHILDS>
where
    R: Removable<MAX_CHILDS>,
{
    #[inline(always)]
    fn index_mut(&mut self, index: R::IDX) -> &mut Self::Output {
        self.remover.index_mut(index)
    }
}

impl<'a, R, const MAX_CHILDS: usize> Mapping<R::IDX> for Recycle<'a, R, MAX_CHILDS>
where
    R: Removable<MAX_CHILDS>,
{
    #[inline(always)]
    fn add_named(&mut self, name: &String) -> R::IDX {
        self.remover.add_named(name)
    }

    #[inline(always)]
    fn add_anon(&mut self) -> R::IDX {
        self.remover.add_anon()
    }

    #[inline(always)]
    fn get_id(&self, name: &String) -> Option<R::IDX> {
        self.remover.get_id(name)
    }

    #[inline(always)]
    fn get_named(&self, id: R::IDX) -> Option<&String> {
        self.remover.get_named(id)
    }
}
