use crate::tree::{index::Indexing, traits::Mapping};

pub trait Domain<I: Indexing> {
    type Type;

    fn var_id(&self) -> I;
    fn iter(&self) -> impl Iterator<Item = Self::Type>;
    fn card(&self) -> usize;
}

pub struct Boolean<I: Indexing> {
    var_id: I,
}

impl<I: Indexing> Domain<I> for Boolean<I> {
    type Type = bool;

    fn var_id(&self) -> I {
        self.var_id
    }

    fn iter(&self) -> impl Iterator<Item = Self::Type> {
        [false, true].into_iter()
    }

    fn card(&self) -> usize {
        2
    }
}

pub struct Integer<I: Indexing> {
    var_id: I,
    card: usize,
}

impl<I: Indexing> Integer<I> {
    pub fn new<M: Mapping<I>>(expr: &M, name: &String, card: usize) -> Self {
        Integer {
            var_id: expr.get_id(name).unwrap(),
            card: card,
        }
    }
}

impl<I: Indexing> Domain<I> for Integer<I> {
    type Type = usize;

    fn var_id(&self) -> I {
        self.var_id
    }

    fn iter(&self) -> impl Iterator<Item = Self::Type> {
        0..self.card
    }

    fn card(&self) -> usize {
        self.card
    }
}
