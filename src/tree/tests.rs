use std::fmt::Debug;
use std::{
    ops::{Index, IndexMut},
    usize,
};

use super::*;

use addr::{Addr, IndexedMutRef, IndexedRef};
use node::{LinkingNode, Node};
use traits::{Mapping, NodeAllocator};
use tree::{NodeValue, Tree};

pub trait Builder<T> {
    fn a(&mut self, value: T) -> Addr;
    fn b<B: Fn(&mut Self) -> Addr>(&mut self, value: T, builder: B) -> Addr;
}

impl<'a, T, U> Builder<U> for IndexedMutRef<'a, T>
where
    U: Copy + Default + Debug + PartialEq,
    T: IndexMut<Addr> + NodeAllocator<Value = U>,
{
    fn a(&mut self, value: U) -> Addr {
        self.array.push(value, &[Addr::NONE, Addr::NONE])
    }

    fn b<B: Fn(&mut Self) -> Addr>(&mut self, value: U, builder: B) -> Addr {
        let child_id = builder(self);
        let idx = self.array.push(value, &[child_id]);
        self.array[child_id].node.replace_parent(idx);
        idx
    }
}

#[test]
fn tree_build() {
    assert_eq!(
        Tree::build(|builder| builder.a(3)),
        Tree {
            named: Default::default(),
            mapping: Default::default(),
            nodes: vec![
                (NodeValue {
                    node: Node {
                        parent: Addr::NONE,
                        childs: [Addr::NONE; 2]
                    },
                    value: 3
                })
            ],
            output: Addr::new(0)
        }
    );

    assert_eq!(
        Tree::build(|builder| builder.b(2, |builder| builder.a(-5))),
        Tree {
            named: Default::default(),
            mapping: Default::default(),
            nodes: vec![
                (NodeValue {
                    node: Node {
                        parent: Addr::new(1),
                        childs: [Addr::NONE; 2]
                    },
                    value: -5
                }),
                (NodeValue {
                    node: Node {
                        parent: Addr::NONE,
                        childs: [Addr::new(0), Addr::NONE]
                    },
                    value: 2
                })
            ],
            output: Addr::new(1)
        }
    );
}

fn compiler_abs_even_tree<'a, const N: usize, T>(
    origin: IndexedRef<Tree<i32, N>>,
    builder: &'a mut IndexedMutRef<Tree<u32, N>>,
) -> Addr
where
    [Addr; N]: Default,
    Tree<u32, N>: IndexMut<Addr> + NodeAllocator<Value = u32>,
{
    let child_id = origin.node.operands()[0];

    if child_id.is_addr() {
        builder.b(origin.value.abs() as u32 * 2, |builder| {
            compiler_abs_even_tree::<N, T>(
                IndexedRef {
                    array: origin.array,
                    idx: child_id,
                },
                builder,
            )
        })
    } else {
        builder.a(origin.value.abs() as u32 * 2)
    }
}

#[test]
fn tree_compile() {
    assert_eq!(
        Tree {
            named: Default::default(),
            mapping: Default::default(),
            nodes: vec![
                (NodeValue {
                    node: Node {
                        parent: Addr::new(1),
                        childs: [Addr::NONE; 2],
                    },
                    value: -5,
                }),
                (NodeValue {
                    node: Node {
                        parent: Addr::NONE,
                        childs: [Addr::new(0), Addr::NONE],
                    },
                    value: 2,
                }),
            ],
            output: Addr::new(1),
        }
        .compile(compiler_abs_even_tree::<2, u32>),
        Tree {
            named: Default::default(),
            mapping: Default::default(),
            nodes: vec![
                (NodeValue {
                    node: Node {
                        parent: Addr::new(1),
                        childs: [Addr::NONE; 2],
                    },
                    value: 10,
                }),
                (NodeValue {
                    node: Node {
                        parent: Addr::NONE,
                        childs: [Addr::new(0), Addr::NONE],
                    },
                    value: 4,
                }),
            ],
            output: Addr::new(1),
        }
    );
}

#[test]
fn mapping() {
    let mut tree: Tree<i32> = Default::default();

    let anon1 = tree.add_anon();
    let a = tree.add_named(&"A".to_string());
    let anon2 = tree.add_anon();
    let b = tree.add_named(&"B".to_string());
    let c = tree.add_named(&"C".to_string());
    let anon3 = tree.add_anon();

    assert_eq!(6, tree.num_named());

    assert_eq!(a, tree.get_id(&"A".to_string()));
    assert_eq!(b, tree.get_id(&"B".to_string()));
    assert_eq!(c, tree.get_id(&"C".to_string()));

    assert_eq!(None, tree.get_named(anon1));
    assert_eq!(Some(&"A".to_string()), tree.get_named(a));
    assert_eq!(None, tree.get_named(anon2));
    assert_eq!(Some(&"B".to_string()), tree.get_named(b));
    assert_eq!(Some(&"C".to_string()), tree.get_named(c));
    assert_eq!(None, tree.get_named(anon3));
}

#[test]
fn addr() {
    assert_eq!(Addr::NONE, Addr { addr: usize::MAX });

    assert_eq!(Addr::NONE, Default::default());

    assert_eq!(Addr { addr: 2 }, Addr::from(Some(2)));
    assert_eq!(Addr::NONE, Addr::from(None as Option<usize>));
    assert_eq!(Some(3), Addr { addr: 3 }.into());

    assert_eq!(Addr { addr: 6 }, Addr::new(6));
    assert_eq!(Addr { addr: 4 }.addr(), 4);

    assert_eq!(Addr { addr: 6 }.is_addr(), true);
    assert_eq!(Addr { addr: usize::MAX }.is_addr(), false);
    assert_eq!(Addr::NONE.is_addr(), false);

    assert_eq!(Addr { addr: 0 }.is_none(), false);
    assert_eq!(Addr { addr: usize::MAX }.is_none(), true);
    assert_eq!(Addr::NONE.is_none(), true);
}

struct Indexed {
    array: [i32; 3],
}

impl Index<Addr> for Indexed {
    type Output = i32;

    fn index(&self, index: Addr) -> &Self::Output {
        &self.array[index.addr()]
    }
}
impl IndexMut<Addr> for Indexed {
    fn index_mut(&mut self, index: Addr) -> &mut Self::Output {
        &mut self.array[index.addr()]
    }
}

#[test]
fn indexed_ref() {
    let array = Indexed { array: [1, 4, 3] };
    assert_eq!(
        *IndexedRef {
            array: &array,
            idx: Addr::new(0),
        },
        1
    );
    assert_eq!(
        *IndexedRef {
            array: &array,
            idx: Addr::new(2),
        },
        3
    );
    assert_eq!(
        *IndexedRef {
            array: &array,
            idx: Addr::new(1),
        },
        4
    );
}

#[test]
fn indexed_mutref() {
    let mut array = Indexed { array: [1, 4, 3] };
    *IndexedMutRef {
        array: &mut array,
        idx: Addr::new(0),
    } = 6;

    *IndexedMutRef {
        array: &mut array,
        idx: Addr::new(1),
    } = 2;

    *IndexedMutRef {
        array: &mut array,
        idx: Addr::new(2),
    } = 4;

    assert_eq!(array.array, [6, 2, 4]);

    assert_eq!(
        *IndexedMutRef {
            array: &mut array,
            idx: Addr::new(0),
        },
        6
    );
    assert_eq!(
        *IndexedMutRef {
            array: &mut array,
            idx: Addr::new(1),
        },
        2
    );
    assert_eq!(
        *IndexedMutRef {
            array: &mut array,
            idx: Addr::new(2),
        },
        4
    );
}
