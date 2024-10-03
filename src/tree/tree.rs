use crate::logic::fragment::{Fragment, FragmentNode};

use super::allocator::{Allocator, Recycle, Remover};
use super::builder::Builder;
use super::index::Indexing;
use super::mapping::Mapping;
use super::node::LinkinNode;

use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::ops::{Index, IndexMut};

pub trait ExpressionTree<F, I, const MAX_CHILDS: usize>
where
    I: Indexing,
    F: Fragment<I, MAX_CHILDS>,
{
    fn build<B: Fn(&mut Builder<Self, MAX_CHILDS>) -> I>(build: B) -> Self;

    fn builder<B: Fn(&mut Builder<Self, MAX_CHILDS>) -> I>(&mut self, build: B);

    fn replace<
        R: Fn(&mut Recycle<'_, Self, MAX_CHILDS>),
        B: Fn(&mut Builder<'_, Recycle<'_, Self, MAX_CHILDS>, MAX_CHILDS>) -> I,
    >(
        &mut self,
        remove: R,
        build: B,
    ) -> I;
}

#[derive(Debug)]
pub struct Tree<F, I: Indexing = u32, const MAX_CHILDS: usize = 2>
where
    F: Fragment<I, MAX_CHILDS>,
{
    named: Vec<Option<String>>,
    mapping: HashMap<String, I>,
    nodes: Vec<F::Node>,
    output: I,
}

impl<F, I, const MAX_CHILDS: usize> ExpressionTree<F, I, MAX_CHILDS> for Tree<F, I, MAX_CHILDS>
where
    I: Indexing,
    F: Fragment<I, MAX_CHILDS>,
{
    fn build<B: Fn(&mut Builder<Self, MAX_CHILDS>) -> I>(build: B) -> Self {
        let mut tree: Tree<F, I, MAX_CHILDS> = Default::default();
        tree.builder(build);
        tree
    }

    fn builder<B: Fn(&mut Builder<Self, MAX_CHILDS>) -> I>(&mut self, build: B) {
        self.output = build(&mut Builder::new(self))
    }

    fn replace<
        R: Fn(&mut Recycle<'_, Self, MAX_CHILDS>),
        B: Fn(&mut Builder<'_, Recycle<'_, Self, MAX_CHILDS>, MAX_CHILDS>) -> I,
    >(
        &mut self,
        remove: R,
        build: B,
    ) -> I {
        let (root, output) = {
            let mut recycle = Recycle::new(self);
            remove(&mut recycle);

            (recycle.root(), build(&mut Builder::new(&mut recycle)))
        };

        if root.is_addr() {
            self[output].replace_parent(root);
            self[root]
                .replace_operand(I::NONE, output)
                .expect("Tree error");
        } else {
            self.output = output;
        }
        output
    }
}

impl<F, I, const MAX_CHILDS: usize> Default for Tree<F, I, MAX_CHILDS>
where
    I: Indexing,
    F: Fragment<I, MAX_CHILDS>,
{
    fn default() -> Self {
        Self {
            named: Default::default(),
            mapping: Default::default(),
            nodes: Default::default(),
            output: I::NONE,
        }
    }
}

impl<F, I, const MAX_CHILDS: usize> Index<I> for Tree<F, I, MAX_CHILDS>
where
    I: Indexing,
    F: Fragment<I, MAX_CHILDS>,
{
    type Output = F::Node;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        &self.nodes[index.addr()]
    }
}

impl<F, I, const MAX_CHILDS: usize> IndexMut<I> for Tree<F, I, MAX_CHILDS>
where
    I: Indexing,
    F: Fragment<I, MAX_CHILDS>,
{
    #[inline]
    fn index_mut<'a>(&'a mut self, index: I) -> &'a mut F::Node {
        &mut self.nodes[index.addr()]
    }
}

impl<F, I: Indexing, const MAX_CHILDS: usize> Allocator<I, F, MAX_CHILDS> for Tree<F, I, MAX_CHILDS>
where
    I: Indexing,
    F: Fragment<I, MAX_CHILDS>,
{
    fn push(&mut self, symbol: F, operands: &[I]) -> I {
        let idx = I::from(self.nodes.len());
        self.nodes.push(F::Node::new(symbol, operands));
        idx
    }

    fn push_node(&mut self, node: &<F as Fragment<I, MAX_CHILDS>>::Node, operands: &[I]) -> I {
        let idx = I::from(self.nodes.len());
        self.nodes.push(node.duplicate(operands));
        idx
    }
}

impl<F, I: Indexing, const MAX_CHILDS: usize> Remover<I> for Tree<F, I, MAX_CHILDS>
where
    I: Indexing,
    F: Fragment<I, MAX_CHILDS>,
{
    fn remove(&mut self, idx: I) -> Result<I, &'static str> {
        let last_idx = I::from(self.nodes.len() - 1);

        // pop last node
        let last_node = self.nodes.pop().expect("Cannot replace node in empty tree");

        // copy reconnect last node if needed
        if idx.addr() < self.nodes.len() {
            for child_idx in last_node.operands() {
                self[child_idx].replace_parent(idx);
            }
            if last_node.parent().is_addr() {
                self[last_node.parent()].replace_operand(last_idx, idx)?;
            }

            self[idx] = last_node;
            Ok(last_idx)
        } else {
            Err("This node doesn't exist")
        }
    }
}

impl<F, I, const MAX_CHILDS: usize> Display for Tree<F, I, MAX_CHILDS>
where
    I: Indexing,
    F: Fragment<I, MAX_CHILDS>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.nodes[self.output.addr()].fmt_display(f, self)
    }
}

impl<F, I: Indexing, const MAX_CHILDS: usize> Mapping<I> for Tree<F, I, MAX_CHILDS>
where
    I: Indexing,
    F: Fragment<I, MAX_CHILDS>,
{
    fn add_named(&mut self, name: &String) -> I {
        if let Some(id) = self.get_id(name) {
            id
        } else {
            let id = I::from(self.named.len());
            self.named.push(Some(name.to_owned()));
            self.mapping.insert(name.to_owned(), id);
            id
        }
    }

    fn add_anon(&mut self) -> I {
        let id = I::from(self.named.len());
        self.named.push(None);
        id
    }

    fn get_id(&self, name: &String) -> Option<I> {
        self.mapping.get(name).cloned()
    }

    fn get_named(&self, id: I) -> Option<&String> {
        if id.addr() < self.named.len() {
            self.named[id.addr()].as_ref()
        } else {
            None
        }
    }
}
