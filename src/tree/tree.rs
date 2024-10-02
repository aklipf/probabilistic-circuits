use crate::logic::fragment::{Fragment, Symbols};

use super::allocator::{Allocator, Recycle};
use super::builder::Builder;
use super::index::Indexing;
use super::mapping::Mapping;
use super::node::Node;
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Tree<S: Symbols, IDX: Indexing = u32, const MAX_CHILDS: usize = 2>
where
    Node<IDX, S, MAX_CHILDS>: Fragment<IDX>,
{
    pub(crate) named: Vec<Option<String>>,
    pub(crate) mapping: HashMap<String, IDX>,
    pub(crate) nodes: Vec<Node<IDX, S, MAX_CHILDS>>,
    pub(crate) output: IDX,
}

impl<S: Symbols, IDX: Indexing, const MAX_CHILDS: usize> Tree<S, IDX, MAX_CHILDS>
where
    Node<IDX, S, MAX_CHILDS>: Fragment<IDX>,
{
    pub fn build<B: Fn(&mut Builder<IDX, S, MAX_CHILDS, Self>) -> IDX>(build: B) -> Self {
        let mut tree: Tree<S, IDX, MAX_CHILDS> = Default::default();
        tree.builder(build);
        tree
    }

    pub fn builder<B: Fn(&mut Builder<IDX, S, MAX_CHILDS, Self>) -> IDX>(&mut self, build: B) {
        self.output = build(&mut Builder { allocator: self })
    }

    pub fn replace<
        R: Fn(&mut Recycle<'_, S, IDX, MAX_CHILDS>),
        B: Fn(&mut Builder<'_, IDX, S, MAX_CHILDS, Recycle<'_, S, IDX, MAX_CHILDS>>) -> IDX,
    >(
        &mut self,
        remove: R,
        build: B,
    ) -> IDX {
        let (root, output) = {
            let mut recycle = Recycle::new(self);
            remove(&mut recycle);

            (
                recycle.root,
                build(&mut Builder {
                    allocator: &mut recycle,
                }),
            )
        };

        if root.is_addr() {
            self[output].parent = root;
            self[root]
                .replace_operand(IDX::NONE, output)
                .expect("Tree error");
        } else {
            self.output = output;
        }
        output
    }
}

impl<S: Symbols, IDX: Indexing, const MAX_CHILDS: usize> Default for Tree<S, IDX, MAX_CHILDS>
where
    Node<IDX, S, MAX_CHILDS>: Fragment<IDX>,
{
    fn default() -> Self {
        Self {
            named: Default::default(),
            mapping: Default::default(),
            nodes: Default::default(),
            output: IDX::NONE,
        }
    }
}

impl<S: Symbols, IDX: Indexing, const MAX_CHILDS: usize> Index<IDX> for Tree<S, IDX, MAX_CHILDS>
where
    Node<IDX, S, MAX_CHILDS>: Fragment<IDX>,
{
    type Output = Node<IDX, S, MAX_CHILDS>;

    #[inline]
    fn index(&self, index: IDX) -> &Self::Output {
        &self.nodes[index.addr()]
    }
}

impl<S: Symbols, IDX: Indexing, const MAX_CHILDS: usize> IndexMut<IDX> for Tree<S, IDX, MAX_CHILDS>
where
    Node<IDX, S, MAX_CHILDS>: Fragment<IDX>,
{
    #[inline]
    fn index_mut<'a>(&'a mut self, index: IDX) -> &'a mut Node<IDX, S, MAX_CHILDS> {
        &mut self.nodes[index.addr()]
    }
}
