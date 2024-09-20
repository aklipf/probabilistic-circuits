use super::builder::Builder;
use super::index::Indexing;
use super::node::Node;
use super::pool::Recycle;
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
            output: IDX::NONE,
        }
    }

    pub fn builder<F: Fn(&mut Builder<'_, IDX, Self>) -> IDX>(&mut self, build: F) {
        self.output = build(&mut Builder { allocator: self })
    }

    pub fn replace<
        F: Fn(&mut Recycle<'_, IDX>),
        G: Fn(&mut Builder<'_, IDX, Recycle<'_, IDX>>) -> IDX,
    >(
        &mut self,
        remove: F,
        build: G,
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
                .input_replace(IDX::NONE, output)
                .expect("Tree error");
        } else {
            self.output = output;
        }
        output
    }
}

impl<IDX: Indexing> Index<IDX> for Tree<IDX> {
    type Output = Node<IDX>;

    #[inline]
    fn index(&self, index: IDX) -> &Self::Output {
        &self.nodes[index.addr()]
    }
}

impl<IDX: Indexing> IndexMut<IDX> for Tree<IDX> {
    #[inline]
    fn index_mut<'a>(&'a mut self, index: IDX) -> &'a mut Node<IDX> {
        &mut self.nodes[index.addr()]
    }
}