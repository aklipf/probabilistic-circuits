use crate::tree::Addr;

pub trait Domain {
    type Type;

    fn iter(&self) -> impl Iterator<Item = Self::Type>;
    fn card(&self) -> usize;
}

pub struct Boolean {}

impl Domain for Boolean {
    type Type = bool;

    fn iter(&self) -> impl Iterator<Item = Self::Type> {
        [false, true].into_iter()
    }

    fn card(&self) -> usize {
        2
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Integer {
    pub vars: Vec<Addr>,
    pub card: usize,
}

impl Domain for Integer {
    type Type = usize;

    fn iter(&self) -> impl Iterator<Item = Self::Type> {
        0..self.card
    }

    fn card(&self) -> usize {
        self.card
    }
}
