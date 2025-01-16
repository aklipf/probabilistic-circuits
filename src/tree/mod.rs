pub mod addr;
pub mod mapping;
pub mod node;
pub mod recycle;
pub mod traits;
pub mod tree;

pub use addr::{Addr, IndexedMutRef, IndexedRef};
pub use mapping::IntoAddr;
pub use node::{LinkingNode, Node};
pub use traits::{Mapping, NodeAllocator};
pub use tree::{NodeValue, Tree};

#[cfg(test)]
mod tests;
