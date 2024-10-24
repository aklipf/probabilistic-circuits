
pub trait Replacer {
    fn replace(&mut self);
}
/*


pub struct NodeRecycler<'a, T, const MAX_CHILDS: usize>
where
    T: Copy + Default + Debug,
{
    pub(super) root: IndexedMutRef<'a, Tree<T, MAX_CHILDS>>,
    pub(super) current: Addr,
}

impl<'a, T, const MAX_CHILDS: usize> NodeRecycler<'a, T, MAX_CHILDS>
where
    T: Copy + Default + Debug,
{
    pub fn cut(tree: &'a mut Tree<T, MAX_CHILDS>, from_node: Addr, until_nodes: &[Addr]) {

        let mut recycle=NodeRecycler {
            root: IndexedMutRef { array: tree, idx: Default::default() },
            current: Default::default(),
        }

        self.root = tree[from_node].0.parent();
        if self.root.is_addr() {
            let _ = self.tree[self.root].0.replace_operand(from_node, Addr::NONE);
        }
        self.tree[from_node].0.unlink_parent();

        for &idx in until_nodes {
            let parent = self.tree[idx].0.parent();
            self.tree[parent].0.remove_operands();
        }
    }

    fn next(&mut self) -> Option<usize> {
        let current_idx = self.current_idx;

        if current_idx.is_none() {
            return None;
        }

        let node = &mut self.tree[self.current_idx];

        match node.0.pop_operand() {
            Some(idx) => {
                self.current_idx = idx;
                self.next()
            }
            None => {
                self.current_idx = node.0.parent();
                node.0.unlink_parent();
                Some(current_idx)
            }
        }
    }
}

impl<'a, T, const MAX_CHILDS: usize> Drop for NodeRecycler<'a, T, MAX_CHILDS>
where
    T: Copy + Default + Debug,
{
    fn drop(&mut self) {
        while let Some(idx) = self.next() {
            self.remove(idx);
        }
    }
}

impl<'a, T, const MAX_CHILDS: usize> Index<Addr> for NodeRecycler<'a, T, MAX_CHILDS>
where
    T: Copy + Default + Debug,
{
    type Output = (Node<MAX_CHILDS>, T);

    #[inline]
    fn index(&self, index: Addr) -> &Self::Output {
        self.tree.index(index)
    }
}

impl<'a, T, const MAX_CHILDS: usize> IndexMut<Addr> for NodeRecycler<'a, T, MAX_CHILDS>
where
    T: Copy + Default + Debug,
{
    #[inline]
    fn index_mut(&mut self, index: Addr) -> &mut (Node<MAX_CHILDS>, T) {
        self.tree.index_mut(index)
    }
}

impl<'a, T, const MAX_CHILDS: usize> NodeAllocator<MAX_CHILDS> for NodeRecycler<'a, T, MAX_CHILDS>
where
    T: Copy + Default + Debug,
{
    type Value = T;

    fn push(&mut self, symbol: Self::Value, operands: &[Addr]) -> Addr {
        match self.next() {
            Some(idx) => {
                self[idx] = (Node::new(operands), symbol);
                idx
            }
            None => self.tree.push(symbol, operands),
        }
    }

    fn remove(&mut self, idx: Addr) -> Result<Addr, &'static str> {
        // replace iterator position if needed
        if self.tree.remove(idx)? == self.current_idx {
            self.current_idx = idx;
        }
        Ok(self.current_idx)
    }
}
 */
