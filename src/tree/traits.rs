use std::fmt::Debug;
use std::ops::{Index, IndexMut};

use super::{addr::Addr, node::LinkingNode, tree::NodeValue};

pub trait Mapping {
    fn add_named(&mut self, name: &String) -> Addr;
    fn add_anon(&mut self) -> Addr;
    fn get_id(&self, name: &String) -> Addr;
    fn get_named(&self, id: Addr) -> Option<&String>;
    fn fmt_named(&self, id: Addr) -> String;
    fn num_named(&self) -> usize;
}

pub trait NodeAllocator:
    Index<Addr, Output = NodeValue<Self::Node, Self::Value>>
    + IndexMut<Addr, Output = NodeValue<Self::Node, Self::Value>>
{
    type Value: Copy + Debug + PartialEq;
    type Node: LinkingNode + Debug + Default + PartialEq;

    fn push(&mut self, symbol: Self::Value, operands: &[Addr]) -> Addr;
    fn remove(&mut self, idx: Addr) -> Result<Addr, &'static str>;
}
