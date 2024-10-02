use std::fmt::Debug;
//use std::ops::IndexMut;
use std::{fmt, ops::Index};

use crate::tree::allocator::Allocator;
use crate::tree::builder::Builder;
use crate::tree::node::Node;
use crate::tree::{index::Indexing, mapping::Mapping};

pub trait OperandsIter<IDX: Indexing> {
    fn operands<T: Index<IDX, Output = Self>>(&self, tree: &T) -> impl Iterator<Item = Self>;
    /*fn operands_mut<T: IndexMut<IDX, Output = Self>>(
        &self,
        tree: &'_ mut T,
    ) -> impl Iterator<Item = &'_ mut Self>;*/
}

pub trait Symbols: Clone + Copy + Debug + Default {}

pub trait Fragment<IDX: Indexing>: Default + OperandsIter<IDX> {
    type Symbols: Symbols;

    fn fmt_display<M: Mapping<IDX = IDX> + Index<IDX, Output = Self>>(
        &self,
        f: &mut fmt::Formatter,
        tree: &M,
    ) -> fmt::Result;
    fn arity(&self) -> usize;
}

impl<IDX: Indexing, S: Symbols, const MAX_CHILDS: usize> OperandsIter<IDX>
    for Node<IDX, S, MAX_CHILDS>
{
    fn operands<T: std::ops::Index<IDX, Output = Self>>(
        &self,
        tree: &T,
    ) -> impl Iterator<Item = Self> {
        self.childs
            .iter()
            .filter_map(|&idx| if idx.is_addr() { Some(tree[idx]) } else { None })
    }

    /*fn operands_mut<T: std::ops::IndexMut<IDX, Output = Self>>(
        &self,
        tree: &'_ mut T,
    ) -> impl Iterator<Item = &'_ mut Self> {
        self.childs.iter().filter_map(|&idx| {
            if idx.is_addr() {
                todo!() //Some(tree.index_mut(idx))
            } else {
                None
            }
        })
    }*/
}
