use std::ops::{Index, IndexMut};

use super::mapping::Mapping;
use super::tree::Tree;

use super::{index::Indexing, node::Node};

pub trait Pool:
    Index<Self::IDX, Output = Node<Self::IDX>> + IndexMut<Self::IDX, Output = Node<Self::IDX>>
where
    Self::IDX: Indexing,
{
    type IDX;
    fn push(&mut self, node: Node<Self::IDX>) -> Self::IDX;
}

impl<IDX: Indexing> Pool for Tree<IDX> {
    type IDX = IDX;

    fn push(&mut self, node: Node<IDX>) -> IDX {
        let idx = IDX::from(self.nodes.len());
        self.nodes.push(node);
        idx
    }
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

pub struct Recycle<'a, IDX: Indexing> {
    pub(super) tree: &'a mut Tree<IDX>,
    pub(super) root: IDX,
    pub(super) current_idx: IDX,
}

impl<'a, IDX: Indexing> Recycle<'a, IDX> {
    pub fn new(tree: &'a mut Tree<IDX>) -> Self {
        Recycle {
            tree: tree,
            root: IDX::NONE,
            current_idx: IDX::NONE,
        }
    }

    pub fn cut(&mut self, from_node: IDX, until_nodes: &[IDX]) {
        self.root = self.tree[from_node].parent;
        if self.root.is_addr() {
            let _ = self.tree[self.root].input_replace(from_node, IDX::NONE);
        }
        self.tree[from_node].parent.unlink();

        for &idx in until_nodes {
            let parent = self.tree[idx].parent();
            self.tree[parent].childs[0].unlink();
            self.tree[parent].childs[1].unlink();
        }
    }

    fn next(&mut self) -> Option<IDX> {
        let current_idx = self.current_idx;

        if current_idx.is_none() {
            return None;
        }

        let node = &mut self.tree[self.current_idx];

        match node.childs.iter_mut().find(|idx| idx.is_addr()) {
            Some(idx) => {
                self.current_idx = *idx;
                idx.unlink();
                self.next()
            }
            None => {
                self.current_idx = node.parent;
                node.parent.unlink();
                Some(current_idx)
            }
        }
    }

    fn replace_with_last(&mut self, idx: IDX) {
        let initial_idx = IDX::from(self.tree.nodes.len() - 1);

        // replace iterator position if needed
        if initial_idx == self.current_idx {
            self.current_idx = idx;
        }

        // pop last node
        let last_node = self
            .tree
            .nodes
            .pop()
            .expect("Cannot replace node in empty tree");

        // copy reconnect last node if needed
        if idx.addr() < self.tree.nodes.len() {
            for child in last_node.childs() {
                if child.is_addr() {
                    self.tree[*child].parent = idx;
                }
            }
            if last_node.parent().is_addr() {
                self.tree[last_node.parent()]
                    .input_replace(initial_idx, idx)
                    .expect("Recycle error");
            }

            self.tree[idx] = last_node;
        }
    }
}

impl<'a, IDX: Indexing> Index<IDX> for Recycle<'a, IDX> {
    type Output = Node<IDX>;

    #[inline]
    fn index(&self, index: IDX) -> &Self::Output {
        &self.tree[index]
    }
}

impl<'a, IDX: Indexing> IndexMut<IDX> for Recycle<'a, IDX> {
    #[inline]
    fn index_mut<'b>(&'b mut self, index: IDX) -> &'b mut Node<IDX> {
        &mut self.tree[index]
    }
}

impl<'a, IDX: Indexing> Pool for Recycle<'a, IDX> {
    type IDX = IDX;

    fn push(&mut self, node: Node<IDX>) -> IDX {
        match self.next() {
            Some(idx) => {
                self.tree[idx] = node;
                idx
            }
            None => {
                let idx = IDX::from(self.tree.nodes.len());
                self.tree.nodes.push(node);
                idx
            }
        }
    }
}

impl<'a, IDX: Indexing> Drop for Recycle<'a, IDX> {
    fn drop(&mut self) {
        while let Some(idx) = self.next() {
            self.replace_with_last(idx)
        }
    }
}

impl<'a, IDX: Indexing> Mapping<IDX> for Recycle<'a, IDX> {
    fn add_named(&mut self, name: &String) -> IDX {
        self.tree.add_named(name)
    }

    fn add_anon(&mut self) -> IDX {
        self.tree.add_anon()
    }

    fn get_id(&self, name: &String) -> Option<IDX> {
        self.tree.get_id(name)
    }

    fn get_named(&self, id: IDX) -> Option<&String> {
        self.tree.get_named(id)
    }
}
