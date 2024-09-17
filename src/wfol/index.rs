use num_traits::PrimInt;
use std::fmt::Debug;

pub trait Indexing: Copy + Clone + Debug + PartialEq {
    const None: Self;

    fn addr(&self) -> usize;
    fn from<T: PrimInt>(addr: T) -> Self;

    fn is_none(&self) -> bool {
        *self == Self::None
    }

    fn offset<T: PrimInt>(&self, n: T) -> Self {
        Self::from((self.addr() as isize + n.to_isize().unwrap()) as usize)
    }

    fn is_addr(&self) -> bool {
        !self.is_none()
    }
}

impl Indexing for u16 {
    const None: Self = u16::MAX;

    fn addr(&self) -> usize {
        *self as usize
    }

    fn from<T: PrimInt>(addr: T) -> Self {
        addr.to_u16().unwrap()
    }
}

impl Indexing for u32 {
    const None: Self = u32::MAX;

    fn addr(&self) -> usize {
        *self as usize
    }

    fn from<T: PrimInt>(addr: T) -> Self {
        addr.to_u32().unwrap()
    }
}

impl Indexing for u64 {
    const None: Self = u64::MAX;

    fn addr(&self) -> usize {
        *self as usize
    }

    fn from<T: PrimInt>(addr: T) -> Self {
        addr.to_u64().unwrap()
    }
}
