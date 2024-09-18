use super::builder::Builder;
use super::index::Indexing;
use super::node::Node;
use std::fmt::Debug;

#[derive(Default, Debug)]
pub struct Tree<IDX: Indexing = u32> {
    pub(crate) variables: Vec<String>,
    pub(crate) predicates: Vec<(String, usize)>,
    pub(crate) nodes: Vec<Node<IDX>>,
    pub(crate) output: IDX,
}

impl<IDX: Indexing> Tree<IDX> {
    pub fn builder<F: Fn(&mut Builder<'_, IDX>) -> IDX>(&mut self, build: F) {
        self.output = build(&mut Builder {
            tree: self,
            parent: IDX::None,
        })
    }

    pub(super) fn push(&mut self, node: Node<IDX>) -> IDX {
        let idx = IDX::from(self.nodes.len());
        self.nodes.push(node);
        idx
    }

    fn remove(&mut self, idx: IDX) {
        let old_idx = IDX::from(self.nodes.len() - 1);

        let node = self.nodes.pop().expect("The tree is empty");

        self.nodes[node.parent().addr()].input_replace(old_idx, idx);
        self.nodes[idx.addr()] = node;
        todo!("fix this");
    }

    pub(super) fn allocate(&mut self) -> IDX {
        self.push(Default::default())
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

    pub(super) fn get(&self, idx: IDX) -> &Node<IDX> {
        &self.nodes[idx.addr()]
    }

    pub(super) fn get_mut(&mut self, idx: IDX) -> &mut Node<IDX> {
        &mut self.nodes[idx.addr()]
    }
}
