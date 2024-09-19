use super::index::Indexing;

use std::fmt::Debug;

#[derive(Clone, Debug)]
pub enum Symbols<IDX: Indexing> {
    Variable { var_id: IDX },
    Not,
    And,
    Or,
    Predicate { pred_id: IDX },
    All { var_id: IDX },
    Any { var_id: IDX },
    None,
}

#[derive(Clone, Debug)]
pub struct Node<IDX: Indexing> {
    pub(super) parent: IDX,
    pub(super) childs: [IDX; 2],
    pub(super) symbol: Symbols<IDX>,
}

impl<IDX: Indexing> Default for Node<IDX> {
    fn default() -> Self {
        Node {
            parent: IDX::NONE,
            childs: [IDX::NONE, IDX::NONE],
            symbol: Symbols::None,
        }
    }
}

impl<IDX: Indexing> Node<IDX> {
    pub fn num_childs(&self) -> usize {
        match self.symbol {
            Symbols::Variable { .. } => {
                if self.childs[0].is_none() {
                    0
                } else {
                    1
                }
            }
            Symbols::Not => 1,
            Symbols::And => 2,
            Symbols::Or => 2,
            Symbols::Predicate { .. } => {
                if self.childs[0].is_none() {
                    0
                } else {
                    1
                }
            }
            Symbols::All { .. } => 1,
            Symbols::Any { .. } => 1,
            _ => 0,
        }
    }

    pub fn childs(&self) -> &[IDX] {
        &self.childs[..self.num_childs()]
    }

    pub fn childs_mut(&mut self) -> &mut [IDX] {
        let num = self.num_childs();
        &mut self.childs[..num]
    }

    pub fn parent(&self) -> IDX {
        self.parent
    }

    pub fn parent_mut(&mut self) -> &mut IDX {
        &mut self.parent
    }

    pub fn symbol(&self) -> &Symbols<IDX> {
        &self.symbol
    }

    pub(super) fn input_replace(&mut self, old: IDX, new: IDX) -> Result<(), String> {
        self.childs_mut()
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
}
