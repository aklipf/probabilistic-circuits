use super::builder::Builder;
use super::index::Indexing;
use super::node::Node;
use std::fmt::Debug;
use std::ops::{Index, IndexMut};

#[derive(Default, Debug)]
pub struct Tree<IDX: Indexing = u32> {
    pub(crate) variables: Vec<String>,
    pub(crate) predicates: Vec<(String, usize)>,
    pub(crate) nodes: Vec<Node<IDX>>,
    pub(crate) output: IDX,
}

impl<IDX: Indexing> Tree<IDX> {
    pub fn new<T: IntoIterator<Item = String>, U: IntoIterator<Item = (String, usize)>>(
        variables: T,
        predicates: U,
    ) -> Self {
        Tree {
            variables: variables.into_iter().collect(),
            predicates: predicates.into_iter().collect(),
            nodes: Default::default(),
            output: IDX::None,
        }
    }

    pub fn builder<F: Fn(&mut Builder<'_, IDX, Self>) -> IDX>(&mut self, build: F) {
        self.output = build(&mut Builder { allocator: self })
    }

    pub(super) fn push(&mut self, node: Node<IDX>) -> IDX {
        let idx = IDX::from(self.nodes.len());
        self.nodes.push(node);
        idx
    }

    pub(super) fn allocate_n(&mut self, n: usize) -> IDX {
        let idx = IDX::from(self.nodes.len());
        self.nodes.resize(self.nodes.len() + n, Default::default());
        idx
    }

    pub(super) fn replace(&mut self, idx: IDX, node: Node<IDX>) -> IDX {
        self.nodes[idx.addr()] = node;
        idx
    }
}

impl<IDX: Indexing> Index<IDX> for Tree<IDX> {
    type Output = Node<IDX>;

    fn index(&self, index: IDX) -> &Self::Output {
        &self.nodes[index.addr()]
    }
}

impl<IDX: Indexing> IndexMut<IDX> for Tree<IDX> {
    fn index_mut<'a>(&'a mut self, index: IDX) -> &'a mut Node<IDX> {
        &mut self.nodes[index.addr()]
    }
}
