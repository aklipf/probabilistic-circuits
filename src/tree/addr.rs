use std::ops::{Index, IndexMut};

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Hash)]
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
    pub array: &'a T,
    pub idx: Addr,
}

impl<'a, T> AsRef<T::Output> for IndexedRef<'a, T>
where
    T: Index<Addr>,
{
    fn as_ref(&self) -> &T::Output {
        &self.array[self.idx]
    }
}

pub struct IndexedMutRef<'a, T>
where
    T: IndexMut<Addr>,
{
    pub array: &'a mut T,
    pub idx: Addr,
}

impl<'a, T> AsRef<T::Output> for IndexedMutRef<'a, T>
where
    T: IndexMut<Addr>,
{
    fn as_ref(&self) -> &T::Output {
        &self.array[self.idx]
    }
}

impl<'a, T> IndexedMutRef<'a, T>
where
    T: IndexMut<Addr>,
{
    pub fn get_ref(&self) -> IndexedRef<'_, T> {
        IndexedRef {
            array: &self.array,
            idx: self.idx,
        }
    }
}

impl<'a, T> AsMut<T::Output> for IndexedMutRef<'a, T>
where
    T: IndexMut<Addr>,
{
    fn as_mut(&mut self) -> &mut T::Output {
        &mut self.array[self.idx]
    }
}
