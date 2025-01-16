use std::fmt::Debug;

use super::addr::Addr;

pub trait LinkingNode {
    fn parent(&self) -> Addr;
    fn unlink_parent(&mut self);
    fn replace_parent(&mut self, idx: Addr);
    fn operands(&self) -> &[Addr];
    fn remove_operands(&mut self);
    fn replace_operand(&mut self, old: Addr, new: Addr) -> Result<(), &'static str>;
    fn replace_operands(&mut self, new: &[Addr]);
    fn pop_operand(&mut self) -> Addr;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Node<const MAX_CHILDS: usize> {
    pub(super) parent: Addr,
    pub(super) childs: [Addr; MAX_CHILDS],
}

impl<const MAX_CHILDS: usize> Default for Node<MAX_CHILDS> {
    #[inline(always)]
    fn default() -> Self {
        Node {
            parent: Addr::NONE,
            childs: [Addr::NONE; MAX_CHILDS],
        }
    }
}

impl<const MAX_CHILDS: usize> Node<MAX_CHILDS> {
    #[inline(always)]
    pub fn new(operands: &[Addr]) -> Self {
        let mut childs = [Addr::NONE; MAX_CHILDS];

        childs
            .iter_mut()
            .zip(operands)
            .for_each(|(dst, src)| *dst = *src);

        Self {
            parent: Addr::NONE,
            childs: childs,
        }
    }
}

impl<const MAX_CHILDS: usize> LinkingNode for Node<MAX_CHILDS> {
    #[inline(always)]
    fn parent(&self) -> Addr {
        self.parent
    }

    #[inline(always)]
    fn unlink_parent(&mut self) {
        self.parent = Addr::NONE;
    }

    #[inline(always)]
    fn replace_parent(&mut self, idx: Addr) {
        self.parent = idx;
    }

    #[inline(always)]
    fn operands(&self) -> &[Addr] {
        &self.childs
    }

    #[inline(always)]
    fn remove_operands(&mut self) {
        self.childs.iter_mut().for_each(|x| *x = Addr::NONE)
    }

    #[inline(always)]
    fn replace_operand(&mut self, old: Addr, new: Addr) -> Result<(), &'static str> {
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
    fn pop_operand(&mut self) -> Addr {
        self.childs
            .iter_mut()
            .find(|idx| idx.is_addr())
            .and_then(|idx| {
                let pop_idx = *idx;
                *idx = Addr::NONE;
                Some(pop_idx)
            })
            .unwrap_or_default()
    }

    #[inline(always)]
    fn replace_operands(&mut self, new: &[Addr]) {
        assert!(new.len() <= MAX_CHILDS);

        let padded = new
            .iter()
            .copied()
            .chain((new.len()..MAX_CHILDS).map(|_| Addr::NONE));

        self.childs
            .iter_mut()
            .zip(padded)
            .for_each(|(dst, src)| *dst = src)
    }
}
