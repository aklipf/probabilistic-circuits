use std::ops::IndexMut;

use super::{Addr, IndexedMutRef, Mapping};

pub trait IntoAddr<T, U>: Copy {
    fn get_addr(&self, map: &mut T) -> Addr;
}

impl<T> IntoAddr<T, Addr> for Addr {
    #[inline(always)]
    fn get_addr(&self, _: &mut T) -> Addr {
        *self
    }
}

impl<T> IntoAddr<T, Addr> for &str
where
    T: Mapping,
{
    #[inline(always)]
    fn get_addr(&self, map: &mut T) -> Addr {
        map.add_named(&self.to_string())
    }
}

impl<'a, T> IntoAddr<IndexedMutRef<'a, T>, Addr> for &str
where
    T: IndexMut<Addr> + Mapping,
{
    #[inline(always)]
    fn get_addr(&self, map: &mut IndexedMutRef<'a, T>) -> Addr {
        map.array.add_named(&self.to_string())
    }
}
