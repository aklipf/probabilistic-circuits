use super::traits::NodeAllocator;

pub struct NodeView<'a, F, I, const MAX_CHILDS: usize>
where
    I: Indexing,
    F: Fragment<I, MAX_CHILDS>,
    Tree<F, I, MAX_CHILDS>: Index<I, Output = F::Node> + Mapping<I>,
{
    pub tree: &'a Tree<F, I, MAX_CHILDS>,
    pub id: I,
    pub symbol: F,
}

impl<'a, F, I, const MAX_CHILDS: usize> Deref for ()
where
    I: Indexing,
    F: Fragment<I, MAX_CHILDS>,
    Tree<F, I, MAX_CHILDS>: Index<I, Output = F::Node>,
{
    type Target = Node<I, F, MAX_CHILDS>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.tree[self.id]
    }
}

impl<'a, F, I, const MAX_CHILDS: usize> NodeView<'a, F, I, MAX_CHILDS>
where
    I: Indexing,
    F: Fragment<I, MAX_CHILDS>,
    Tree<F, I, MAX_CHILDS>: Index<I, Output = F::Node>,
{
    #[inline(always)]
    pub fn left(&self) -> NodeView<'a, F, I, MAX_CHILDS> {
        let child_id = self.operand()[0];
        NodeView {
            tree: self.tree,
            id: child_id,
            symbol: self.tree[child_id].symbol(),
        }
    }

    #[inline(always)]
    pub fn right(&'a self) -> NodeView<'a, F, I, MAX_CHILDS> {
        let child_id = self.operand()[1];
        NodeView {
            tree: self.tree,
            id: child_id,
            symbol: self.tree[child_id].symbol(),
        }
    }

    #[inline(always)]
    pub fn input(&'a self) -> NodeView<'a, F, I, MAX_CHILDS> {
        let child_id = self.operand()[0];
        NodeView {
            tree: self.tree,
            id: child_id,
            symbol: self.tree[child_id].symbol(),
        }
    }

    #[inline(always)]
    pub fn output(&'a self) -> Option<NodeView<'a, F, I, MAX_CHILDS>> {
        let parent_id = self.parent();
        if parent_id.is_addr() {
            Some(NodeView {
                tree: self.tree,
                id: parent_id,
                symbol: self.tree[parent_id].symbol(),
            })
        } else {
            None
        }
    }
}
 
/*pub trait View<T>: Sized {
    fn output(&self) -> Option<Self>;
    fn value(&self) -> T;
}*/

pub struct NodeView<'a, A> {
    pub(super) allocator: &'a mut A,
    pub(super) idx: usize,
}

impl<'a, A, const MAX_CHILDS: usize> NodeView<'a, A>
where
    A: NodeAllocator<MAX_CHILDS>,
{
    #[inline(always)]
    fn left(self) -> Result<Self, &'static str> {
        let node_id = self.allocator[self.idx].0.operands()[0];

        if node_id.is_addr() {
            Ok(NodeView {
                allocator: self.allocator,
                idx: node_id,
            })
        } else {
            Err("this node have no left child")
        }
    }

    #[inline(always)]
    fn right(self) -> Result<Self, &'static str> {
        let node_id = self.allocator[self.idx].0.operands()[1];

        if node_id.is_addr() {
            Ok(NodeView {
                allocator: self.allocator,
                idx: node_id,
            })
        } else {
            Err("this node have no right child")
        }
    }

    #[inline(always)]
    fn inner(self) -> Result<Self, &'static str> {
        let node_id = self.allocator[self.idx].0.operands()[0];

        if node_id.is_addr() {
            Ok(NodeView {
                allocator: self.allocator,
                idx: node_id,
            })
        } else {
            Err("this node have no inner child")
        }
    }

    #[inline(always)]
    fn output(self) -> Option<Self> {
        let parent_id = self.allocator[self.idx].0.parent();

        if parent_id.is_addr() {
            Some(NodeView {
                allocator: self.allocator,
                idx: parent_id,
            })
        } else {
            None
        }
    }

    #[inline(always)]
    fn value(&self) -> A::Value {
        self.allocator[self.idx].1
    }
}

impl<'a, N, A, I, T> View<T> for (A, I)
where
    A: NodeAllocator<Node = N, Idx = I, Value = T>,
    N: LinkingNode<I>,
    I: Indexing,
    T: Copy,
{
    #[inline(always)]
    pub fn left(&self) -> NodeView<'a, F, I, MAX_CHILDS> {
        let child_id = self.operand()[0];
        NodeView {
            tree: self.tree,
            id: child_id,
            symbol: self.tree[child_id].symbol(),
        }
    }

    #[inline(always)]
    pub fn right(&'a self) -> NodeView<'a, F, I, MAX_CHILDS> {
        let child_id = self.operand()[1];
        NodeView {
            tree: self.tree,
            id: child_id,
            symbol: self.tree[child_id].symbol(),
        }
    }

    #[inline(always)]
    pub fn input(&'a self) -> NodeView<'a, F, I, MAX_CHILDS> {
        let child_id = self.operand()[0];
        NodeView {
            tree: self.tree,
            id: child_id,
            symbol: self.tree[child_id].symbol(),
        }
    }

    #[inline(always)]
    fn output((&mut tree,idx)) -> Option<Self> {
        let parent_id = self.0[self.1].0.parent();

        if parent_id.is_addr() {
            Some((self.0, parent_id))
        } else {
            None
        }
    }

    fn value(&self) -> T {
        self.0[self.1].1
    }
}
