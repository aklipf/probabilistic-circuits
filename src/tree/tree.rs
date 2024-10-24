use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::{Index, IndexMut};

use super::addr::{Addr, IndexedMutRef, IndexedRef};
use super::node::{LinkingNode, Node};
use super::traits::{Mapping, NodeAllocator};

#[derive(Debug, Default, PartialEq)]
pub struct NodeValue<N: LinkingNode, T: Copy + Debug + PartialEq> {
    pub node: N,
    pub value: T,
}

#[derive(Debug, PartialEq)]
pub struct Tree<T, const MAX_CHILDS: usize = 2>
where
    T: Copy + Debug + PartialEq,
{
    pub(super) named: Vec<Option<String>>,
    pub(super) mapping: HashMap<String, usize>,
    pub(super) nodes: Vec<NodeValue<Node<MAX_CHILDS>, T>>,
    pub(super) output: Addr,
}

impl<T, const MAX_CHILDS: usize> Default for Tree<T, MAX_CHILDS>
where
    T: Copy + Debug + PartialEq,
{
    fn default() -> Self {
        Self {
            named: Default::default(),
            mapping: Default::default(),
            nodes: Default::default(),
            output: Default::default(),
        }
    }
}

impl<T, const MAX_CHILDS: usize> Tree<T, MAX_CHILDS>
where
    T: Copy + Debug + PartialEq,
{
    pub fn output<'a>(&'a self) -> IndexedRef<'a, Self> {
        let output = self.output;
        IndexedRef {
            array: &self,
            idx: output,
        }
    }

    pub fn build<B: Fn(&mut IndexedMutRef<Self>) -> Addr>(builder: B) -> Self {
        let mut tree: Self = Default::default();

        tree.output = builder(&mut IndexedMutRef {
            array: &mut tree,
            idx: Addr::NONE,
        });

        tree
    }

    pub fn builder<B: Fn(&mut IndexedMutRef<Self>) -> Addr>(&mut self, builder: B) {
        self.output = builder(&mut IndexedMutRef {
            array: self,
            idx: Addr::NONE,
        })
    }

    pub fn compile<
        U: Copy + Debug + PartialEq,
        const N: usize,
        B: Fn(IndexedRef<Self>, &mut IndexedMutRef<Tree<U, N>>) -> Addr,
    >(
        &self,
        builder: B,
    ) -> Tree<U, N> {
        let mut tree: Tree<U, N> = Default::default();

        tree.output = builder(
            self.output(),
            &mut IndexedMutRef {
                array: &mut tree,
                idx: Addr::NONE,
            },
        );

        tree
    }

    /*fn replace<B: Fn(IndexedMutRef<NodeRecycler<T,MAX_CHILDS>>) -> Addr>(
        &mut self,
        from_node: Addr,
        until_nodes: &[Addr],
        builder: B
    ) {
        let mut recycle = NodeRecycler::cut(self);
        recycle.cut(from_node, until_nodes);
        let root = recycle.root();
        (recycle, root)

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

    }*/
}

impl<T, const MAX_CHILDS: usize> Index<Addr> for Tree<T, MAX_CHILDS>
where
    T: Copy + Debug + PartialEq,
{
    type Output = NodeValue<Node<MAX_CHILDS>, T>;

    #[inline]
    fn index(&self, index: Addr) -> &Self::Output {
        &self.nodes[index.addr()]
    }
}

impl<T, const MAX_CHILDS: usize> IndexMut<Addr> for Tree<T, MAX_CHILDS>
where
    T: Copy + Debug + PartialEq,
{
    #[inline]
    fn index_mut<'a>(&'a mut self, index: Addr) -> &'a mut NodeValue<Node<MAX_CHILDS>, T> {
        &mut self.nodes[index.addr()]
    }
}

impl<T, const MAX_CHILDS: usize> NodeAllocator for Tree<T, MAX_CHILDS>
where
    T: Copy + Debug + PartialEq,
{
    type Value = T;
    type Node = Node<MAX_CHILDS>;

    fn push(&mut self, symbol: T, operands: &[Addr]) -> Addr {
        let idx = self.nodes.len();
        self.nodes.push(NodeValue {
            node: Node::new(operands),
            value: symbol,
        });
        Addr::new(idx)
    }

    fn remove(&mut self, idx: Addr) -> Result<Addr, &'static str> {
        let last_idx = Addr::new(self.nodes.len() - 1);

        // pop last node
        let last_node = self.nodes.pop().expect("Cannot replace node in empty tree");

        // copy reconnect last node if needed
        if idx.addr() < self.nodes.len() {
            for &child_idx in last_node.node.operands() {
                self[child_idx].node.replace_parent(idx);
            }
            if last_node.node.parent().is_addr() {
                self[last_node.node.parent()]
                    .node
                    .replace_operand(last_idx, idx)?;
            }

            self[idx] = last_node;
            Ok(last_idx)
        } else {
            Err("This node doesn't exist")
        }
    }
}

impl<T, const MAX_CHILDS: usize> Mapping for Tree<T, MAX_CHILDS>
where
    T: Copy + Debug + PartialEq,
{
    fn add_named(&mut self, name: &String) -> Addr {
        let id = self.get_id(name);
        if id.is_addr() {
            id
        } else {
            let id = self.named.len();
            self.named.push(Some(name.to_owned()));
            self.mapping.insert(name.to_owned(), id);
            Addr::new(id)
        }
    }

    fn add_anon(&mut self) -> Addr {
        let id = self.named.len();
        self.named.push(None);
        Addr::new(id)
    }

    fn get_id(&self, name: &String) -> Addr {
        self.mapping.get(name).into()
    }

    fn get_named(&self, id: Addr) -> Option<&String> {
        if id.addr() < self.named.len() {
            self.named[id.addr()].as_ref()
        } else {
            None
        }
    }

    fn num_named(&self) -> usize {
        self.named.len()
    }

    fn fmt_named(&self, id: Addr) -> String {
        match self.get_named(id) {
            Some(name) => name.clone(),
            None => format!("x{}", id.addr() + 1),
        }
    }
}
