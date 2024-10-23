use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Deref;

pub trait FragmentNode<I, F, const MAX_CHILDS: usize>: Display + Deref<Target = F>
where
    I: Indexing,
    F: Fragment<I, MAX_CHILDS>,
{
    fn arity(&self) -> usize;
}

pub trait Fragment: Clone + Copy + Debug + Default {
    fn symbol(&self) -> Self {
        *self
    }
}
