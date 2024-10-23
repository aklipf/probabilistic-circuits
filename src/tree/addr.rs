use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Addr {
    pub addr: usize,
}

impl Default for Addr {
    fn default() -> Self {
        Self::NONE
    }
}

impl From<Option<usize>> for Addr {
    fn from(value: Option<usize>) -> Self {
        Addr {
            addr: value.unwrap_or(Self::NONE.addr),
        }
    }
}

impl From<Option<&usize>> for Addr {
    fn from(value: Option<&usize>) -> Self {
        Addr {
            addr: *value.unwrap_or(&Self::NONE.addr),
        }
    }
}

impl Into<Option<usize>> for Addr {
    fn into(self) -> Option<usize> {
        if self.is_addr() {
            Some(self.addr)
        } else {
            None
        }
    }
}

impl Addr {
    pub const NONE: Self = Addr { addr: usize::MAX };

    #[inline(always)]
    pub const fn new(addr: usize) -> Self {
        Addr { addr: addr }
    }

    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Self::NONE
    }

    #[inline(always)]
    pub fn is_addr(&self) -> bool {
        *self != Self::NONE
    }

    #[inline(always)]
    pub fn addr(&self) -> usize {
        self.addr
    }
}

pub struct IndexedRef<'a, T>
where
    T: Index<Addr>,
{
    pub(super) array: &'a T,
    pub(super) idx: Addr,
}

impl<'a, T> Deref for IndexedRef<'a, T>
where
    T: Index<Addr>,
{
    type Target = T::Output;

    fn deref(&self) -> &Self::Target {
        &self.array[self.idx]
    }
}

pub struct IndexedMutRef<'a, T>
where
    T: IndexMut<Addr>,
{
    pub(super) array: &'a mut T,
    pub(super) idx: Addr,
}

impl<'a, T> Deref for IndexedMutRef<'a, T>
where
    T: IndexMut<Addr>,
{
    type Target = T::Output;

    fn deref(&self) -> &Self::Target {
        &self.array[self.idx]
    }
}

impl<'a, T> DerefMut for IndexedMutRef<'a, T>
where
    T: IndexMut<Addr>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.array[self.idx]
    }
}
