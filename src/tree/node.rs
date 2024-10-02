use crate::logic::fragment::Symbols;

use super::index::Indexing;

use std::fmt::Debug;

#[derive(Clone, Copy, Debug)]
pub struct Node<IDX: Indexing, S: Symbols, const MAX_CHILDS: usize> {
    pub(crate) parent: IDX,
    pub(crate) childs: [IDX; MAX_CHILDS],
    pub(crate) symbol: S,
}

impl<IDX: Indexing, S: Symbols, const MAX_CHILDS: usize> Default for Node<IDX, S, MAX_CHILDS> {
    fn default() -> Self {
        Node {
            parent: IDX::NONE,
            childs: [IDX::NONE; MAX_CHILDS],
            symbol: S::default(),
        }
    }
}

impl<IDX: Indexing, S: Symbols, const MAX_CHILDS: usize> Node<IDX, S, MAX_CHILDS> {
    pub(crate) fn replace_operand(&mut self, old: IDX, new: IDX) -> Result<(), String> {
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

    pub(crate) fn childs_idx(&self) -> impl Iterator<Item = &IDX> {
        self.childs.iter().filter(|&idx| idx.is_addr())
    }
}
