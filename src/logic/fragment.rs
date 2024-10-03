use std::fmt::Debug;
//use std::ops::IndexMut;
use std::{fmt, ops::Index};

use crate::tree::node::LinkinNode;
use crate::tree::{index::Indexing, mapping::Mapping};

pub trait FragmentNode<I, F, const MAX_CHILDS: usize>: LinkinNode<I>
where
    I: Indexing,
{
    fn fmt_display<M: Mapping<IDX = I> + Index<I, Output = Self>>(
        &self,
        f: &mut fmt::Formatter,
        tree: &M,
    ) -> fmt::Result;
    fn arity(&self) -> usize;
    fn new(symbol: F, operands: &[I]) -> Self;
    fn duplicate(&self, operands: &[I]) -> Self;
}

pub trait Fragment<I, const MAX_CHILDS: usize>: Clone + Copy + Debug + Default
where
    I: Indexing,
{
    type Node: FragmentNode<I, Self, MAX_CHILDS>;
}
