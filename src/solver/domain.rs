use crate::tree::{Addr, Mapping};

pub trait Domain {
    type Type;

    fn var_id(&self) -> Addr;
    fn iter(&self) -> impl Iterator<Item = Self::Type>;
    fn card(&self) -> usize;
}

pub struct Boolean {
    var_id: Addr,
}

impl Domain for Boolean {
    type Type = bool;

    fn var_id(&self) -> Addr {
        self.var_id
    }

    fn iter(&self) -> impl Iterator<Item = Self::Type> {
        [false, true].into_iter()
    }

    fn card(&self) -> usize {
        2
    }
}

pub struct Integer {
    var_id: Addr,
    card: usize,
}

impl Integer {
    pub fn new<M: Mapping>(expr: &M, name: &String, card: usize) -> Self {
        Integer {
            var_id: expr.get_id(name),
            card: card,
        }
    }
}

impl Domain for Integer {
    type Type = usize;

    fn var_id(&self) -> Addr {
        self.var_id
    }

    fn iter(&self) -> impl Iterator<Item = Self::Type> {
        0..self.card
    }

    fn card(&self) -> usize {
        self.card
    }
}
