use super::index::Indexing;
use std::fmt::Debug;

pub trait LinkinNode<I: Indexing>: Default + Clone + Copy + Debug {
    fn parent(&self) -> I;
    fn unlink_parent(&mut self);
    fn replace_parent(&mut self, idx: I);
    fn operands(&self) -> impl Iterator<Item = I>;
    fn remove_operands(&mut self);
    fn replace_operand(&mut self, old: I, new: I) -> Result<(), &'static str>;
    fn pop_operand(&mut self) -> Option<I>;
}

#[derive(Clone, Copy, Debug)]
pub struct Node<I: Indexing, F: Clone + Copy + Debug + Default, const MAX_CHILDS: usize> {
    parent: I,
    childs: [I; MAX_CHILDS],
    symbol: F,
}

impl<IDX: Indexing, F: Clone + Copy + Debug + Default, const MAX_CHILDS: usize> Default
    for Node<IDX, F, MAX_CHILDS>
{
    #[inline(always)]
    fn default() -> Self {
        Node {
            parent: IDX::NONE,
            childs: [IDX::NONE; MAX_CHILDS],
            symbol: F::default(),
        }
    }
}

impl<I: Indexing, F: Clone + Copy + Debug + Default, const MAX_CHILDS: usize>
    Node<I, F, MAX_CHILDS>
{
    #[inline(always)]
    pub fn new(symbol: F, operands: &[I]) -> Self {
        let mut childs = [I::NONE; MAX_CHILDS];

        childs
            .iter_mut()
            .zip(operands)
            .for_each(|(dst, src)| *dst = *src);

        Self {
            parent: I::NONE,
            childs: childs,
            symbol: symbol,
        }
    }

    #[inline(always)]
    pub fn symbol(&self) -> F {
        self.symbol
    }

    #[inline(always)]
    pub fn symbol_mut(&mut self) -> &mut F {
        &mut self.symbol
    }
}

impl<I: Indexing, F: Clone + Copy + Debug + Default, const MAX_CHILDS: usize> LinkinNode<I>
    for Node<I, F, MAX_CHILDS>
{
    #[inline(always)]
    fn parent(&self) -> I {
        self.parent
    }

    #[inline(always)]
    fn unlink_parent(&mut self) {
        self.parent = I::NONE;
    }

    #[inline(always)]
    fn replace_parent(&mut self, idx: I) {
        self.parent = idx;
    }

    #[inline(always)]
    fn operands(&self) -> impl Iterator<Item = I> {
        self.childs
            .iter()
            .filter_map(|idx| if idx.is_addr() { Some(*idx) } else { None })
    }

    #[inline(always)]
    fn remove_operands(&mut self) {
        self.childs.iter_mut().for_each(|x| *x = I::NONE)
    }

    #[inline(always)]
    fn replace_operand(&mut self, old: I, new: I) -> Result<(), &'static str> {
        self.childs
            .iter_mut()
            .find_map(|x| {
                if *x == old {
                    *x = new;
                    Some(())
                } else {
                    None
                }
            })
            .ok_or("input not found")?;
        Ok(())
    }

    #[inline(always)]
    fn pop_operand(&mut self) -> Option<I> {
        self.childs
            .iter_mut()
            .find(|idx| idx.is_addr())
            .and_then(|idx| {
                let pop_idx = *idx;
                idx.unlink();
                Some(pop_idx)
            })
    }
}
