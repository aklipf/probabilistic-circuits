use std::ops::{Deref, DerefMut, Index, IndexMut};

use crate::logic::fragment::{Fragment, FragmentNode};

use super::allocator::Allocator;
use super::index::Indexing;
use super::mapping::Mapping;
use super::node::LinkinNode;
use super::tree::Tree;

pub trait Buildable<const MAX_CHILDS: usize>:
    Allocator<
        MAX_CHILDS,
        IDX = <Self as Buildable<MAX_CHILDS>>::IDX,
        Fragment = <Self as Buildable<MAX_CHILDS>>::Fragment,
    > + Mapping<<Self as Buildable<MAX_CHILDS>>::IDX>
    + Index<
        <Self as Buildable<MAX_CHILDS>>::IDX,
        Output = <<Self as Buildable<MAX_CHILDS>>::Fragment as Fragment<
            <Self as Buildable<MAX_CHILDS>>::IDX,
            MAX_CHILDS,
        >>::Node,
    > + IndexMut<<Self as Buildable<MAX_CHILDS>>::IDX>
{
    type IDX: Indexing;
    type Fragment: Fragment<<Self as Buildable<MAX_CHILDS>>::IDX, MAX_CHILDS>;
}

impl<F, I, const MAX_CHILDS: usize> Buildable<MAX_CHILDS> for Tree<F, I, MAX_CHILDS>
where
    I: Indexing,
    F: Fragment<I, MAX_CHILDS>,
{
    type IDX = I;
    type Fragment = F;
}

#[derive(Debug)]
pub struct Builder<'a, B, const MAX_CHILDS: usize>
where
    B: Buildable<MAX_CHILDS>,
{
    buildable: &'a mut B,
}

/*
impl<
        'a,
        IDX: Indexing,
        F: Fragment<IDX>,
        const MAX_CHILDS: usize,
        P: Allocator<F, MAX_CHILDS, IDX = IDX> + Mapping<IDX>,
    > Builder<'a, IDX, P>
{
    #[inline]
    fn pred_arg(&mut self, vars_id: &[IDX]) -> IDX {
        if vars_id.len() == 1 {
            return self.push(Symbols::Variable { var_id: vars_id[0] });
        }

        self.push_unary(Symbols::Variable { var_id: vars_id[0] }, |builder| {
            builder.pred_arg(&vars_id[1..])
        })
    }

    #[inline]
    pub fn pred(&mut self, pred_id: IDX, vars_id: &[IDX]) -> IDX {
        if vars_id.len() == 0 {
            return self.push(Symbols::Predicate { pred_id: pred_id });
        }

        self.push_unary(Symbols::Predicate { pred_id: pred_id }, |builder| {
            builder.pred_arg(vars_id)
        })
    }

    #[inline]
    pub fn every<F: Fn(&mut Self) -> IDX>(&mut self, var_id: IDX, inner: F) -> IDX {
        self.push_unary(Symbols::Every { var_id: var_id }, inner)
    }

    #[inline]
    pub fn exist<F: Fn(&mut Self) -> IDX>(&mut self, var_id: IDX, inner: F) -> IDX {
        self.push_unary(Symbols::Exist { var_id: var_id }, inner)
    }

    #[inline]
    pub fn every_n<F: Fn(&mut Self) -> IDX>(&mut self, var_id: &[IDX], inner: F) -> IDX {
        let next_idx = inner(self);
        let mut idx = IDX::NONE;

        for &id in var_id.iter().rev() {
            idx = self.allocator.push(Node {
                parent: IDX::NONE,
                childs: [next_idx, IDX::NONE],
                symbol: Symbols::Every { var_id: id },
            });
            self.allocator[next_idx].parent = idx;
        }

        idx
    }

    #[inline]
    pub fn exist_n<F: Fn(&mut Self) -> IDX>(&mut self, var_id: &[IDX], inner: F) -> IDX {
        let next_idx = inner(self);
        let mut idx = IDX::NONE;

        for &id in var_id.iter().rev() {
            idx = self.allocator.push(Node {
                parent: IDX::NONE,
                childs: [next_idx, IDX::NONE],
                symbol: Symbols::Exist { var_id: id },
            });
            self.allocator[next_idx].parent = idx;
        }

        idx
    }

    #[inline]
    pub fn connect(&mut self, node_id: IDX) -> IDX {
        node_id
    }

    #[inline]
    pub fn copy(&mut self, node_id: IDX) -> IDX {
        let node = self.allocator[node_id];
        let idx = self.allocator.push(node);
        for i in 0..2 {
            if node.childs[i].is_addr() {
                let res_idx = self.copy(node.childs[i]);
                self.allocator[res_idx].parent = idx;
            }
        }
        idx
    }
}

*/

impl<'a, B, const MAX_CHILDS: usize> Builder<'a, B, MAX_CHILDS>
where
    B: Buildable<MAX_CHILDS>,
{
    pub fn new(buildable: &'a mut B) -> Self {
        Self { buildable }
    }

    #[inline]
    pub fn connect(
        &mut self,
        node_id: <B as Buildable<MAX_CHILDS>>::IDX,
    ) -> <B as Buildable<MAX_CHILDS>>::IDX {
        node_id
    }

    #[inline]
    pub fn copy(
        &mut self,
        node_id: <B as Buildable<MAX_CHILDS>>::IDX,
    ) -> <B as Buildable<MAX_CHILDS>>::IDX {
        let node = self.buildable[node_id];
        let mut childs_ids = [<B as Buildable<MAX_CHILDS>>::IDX::NONE; MAX_CHILDS];
        childs_ids
            .iter_mut()
            .zip(node.operands())
            .for_each(|(dst, src)| *dst = self.copy(src));

        self.buildable
            .push_node(&node.duplicate(&[]), &childs_ids[..])
    }
}

impl<'a, B, const MAX_CHILDS: usize> Deref for Builder<'a, B, MAX_CHILDS>
where
    B: Buildable<MAX_CHILDS>,
{
    type Target = B;

    fn deref(&self) -> &Self::Target {
        self.buildable
    }
}

impl<'a, B, const MAX_CHILDS: usize> DerefMut for Builder<'a, B, MAX_CHILDS>
where
    B: Buildable<MAX_CHILDS>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.buildable
    }
}
