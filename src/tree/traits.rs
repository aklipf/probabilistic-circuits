use std::ops::{Index, IndexMut};

use crate::logic::fragment::Fragment;

use super::index::Indexing;

pub trait Buildable<const MAX_CHILDS: usize>:
    Allocator<Self::IDX, Self::Fragment, MAX_CHILDS>
    + Mapping<Self::IDX>
    + Index<Self::IDX, Output = <Self::Fragment as Fragment<Self::IDX, MAX_CHILDS>>::Node>
    + IndexMut<Self::IDX>
{
    type IDX: Indexing;
    type Fragment: Fragment<Self::IDX, MAX_CHILDS>;
}
pub trait Removable<const MAX_CHILDS: usize>: Buildable<MAX_CHILDS> + Remover<Self::IDX> {}

pub trait Mapping<I: Indexing> {
    fn add_named(&mut self, name: &String) -> I;
    fn add_anon(&mut self) -> I;
    fn get_id(&self, name: &String) -> Option<I>;
    fn get_named(&self, id: I) -> Option<&String>;
}

pub trait Allocator<I, F, const MAX_CHILDS: usize>
where
    I: Indexing,
    F: Fragment<I, MAX_CHILDS>,
{
    fn push(&mut self, symbol: F, operands: &[I]) -> I;
    fn push_node(&mut self, node: &<F as Fragment<I, MAX_CHILDS>>::Node, operands: &[I]) -> I;
}

pub trait Remover<I: Indexing> {
    fn remove(&mut self, idx: I) -> Result<I, &'static str>;
}
