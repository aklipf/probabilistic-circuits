use std::collections::HashMap;

use crate::{
    logic::{
        first_order::{ground::Grounded, Args, FOLogic, FORef, FirstOrderTree},
        propositional::{PLogic, PRef, PropositionalTree},
    },
    solver::domain::Integer,
    tree::{Addr, IndexedMutRef, IndexedRef},
};

use super::{PCMut, ProbabilisticCircuitTree};

fn p2c_recusive(
    src: IndexedRef<PropositionalTree>,
    dst: &mut IndexedMutRef<ProbabilisticCircuitTree>,
    reverse: bool,
) -> Addr {
    match src.as_ref().value {
        PLogic::Variable { id } => {
            if reverse {
                dst.not_var(id)
            } else {
                dst.var(id)
            }
        }
        PLogic::Not => p2c_recusive(src.inner(), dst, !reverse),
        PLogic::And => {
            if reverse {
                dst.sum(
                    |left| p2c_recusive(src.left(), left, reverse),
                    |right| p2c_recusive(src.right(), right, reverse),
                )
            } else {
                dst.prod(
                    |left| p2c_recusive(src.left(), left, reverse),
                    |right| p2c_recusive(src.right(), right, reverse),
                )
            }
        }
        PLogic::Or => {
            if reverse {
                dst.prod(
                    |left| p2c_recusive(src.left(), left, reverse),
                    |right| p2c_recusive(src.right(), right, reverse),
                )
            } else {
                dst.sum(
                    |left| p2c_recusive(src.left(), left, reverse),
                    |right| p2c_recusive(src.right(), right, reverse),
                )
            }
        }
    }
}

pub fn propositional_to_circuit(tree: &PropositionalTree) -> ProbabilisticCircuitTree {
    tree.compile(|src, dst| {
        dst.array.copy_named(src.array);
        p2c_recusive(src, dst, false)
    })
}

fn fo2c_recusive(
    src: IndexedRef<FirstOrderTree>,
    dst: &mut IndexedMutRef<ProbabilisticCircuitTree>,
    grounded: &Vec<Grounded>,
    domains: &[Integer],
    values: &HashMap<Addr, usize>,
    reverse: bool,
) -> Addr {
    match src.as_ref().value {
        FOLogic::Not => fo2c_recusive(
            src.inner().unwrap(),
            dst,
            grounded,
            domains,
            values,
            !reverse,
        ),
        FOLogic::And => {
            if reverse {
                dst.sum(
                    |left| {
                        fo2c_recusive(
                            src.left().unwrap(),
                            left,
                            grounded,
                            domains,
                            values,
                            reverse,
                        )
                    },
                    |right| {
                        fo2c_recusive(
                            src.right().unwrap(),
                            right,
                            grounded,
                            domains,
                            values,
                            reverse,
                        )
                    },
                )
            } else {
                dst.prod(
                    |left| {
                        fo2c_recusive(
                            src.left().unwrap(),
                            left,
                            grounded,
                            domains,
                            values,
                            reverse,
                        )
                    },
                    |right| {
                        fo2c_recusive(
                            src.right().unwrap(),
                            right,
                            grounded,
                            domains,
                            values,
                            reverse,
                        )
                    },
                )
            }
        }
        FOLogic::Or => {
            if reverse {
                dst.prod(
                    |left| {
                        fo2c_recusive(
                            src.left().unwrap(),
                            left,
                            grounded,
                            domains,
                            values,
                            reverse,
                        )
                    },
                    |right| {
                        fo2c_recusive(
                            src.right().unwrap(),
                            right,
                            grounded,
                            domains,
                            values,
                            reverse,
                        )
                    },
                )
            } else {
                dst.sum(
                    |left| {
                        fo2c_recusive(
                            src.left().unwrap(),
                            left,
                            grounded,
                            domains,
                            values,
                            reverse,
                        )
                    },
                    |right| {
                        fo2c_recusive(
                            src.right().unwrap(),
                            right,
                            grounded,
                            domains,
                            values,
                            reverse,
                        )
                    },
                )
            }
        }
        FOLogic::Predicate { id } => {
            let ground = grounded.iter().find(|&g| g.id == id).unwrap();
            let addr = ground.get_id(src.args().map(|addr| values.get(&addr).unwrap()));
            dst.var(addr)
        }
        FOLogic::Universal { id } => {
            let domain = domains.iter().find(|&x| x.vars.contains(&id)).unwrap();

            if reverse {
                dst.sum_n(&mut (0..domain.card), |inner, value| {
                    let mut current_values = values.clone();
                    current_values.insert(id, value);

                    (
                        fo2c_recusive(
                            src.inner().unwrap(),
                            inner,
                            grounded,
                            domains,
                            &current_values,
                            reverse,
                        ),
                        1.0,
                    )
                })
            } else {
                dst.prod_n(&mut (0..domain.card), |inner, value| {
                    let mut current_values = values.clone();
                    current_values.insert(id, value);

                    fo2c_recusive(
                        src.inner().unwrap(),
                        inner,
                        grounded,
                        domains,
                        &current_values,
                        reverse,
                    )
                })
            }
        }
        FOLogic::Existential { id } => {
            let domain = domains.iter().find(|&x| x.vars.contains(&id)).unwrap();

            if reverse {
                dst.prod_n(&mut (0..domain.card), |inner, value| {
                    let mut current_values = values.clone();
                    current_values.insert(id, value);

                    fo2c_recusive(
                        src.inner().unwrap(),
                        inner,
                        grounded,
                        domains,
                        &current_values,
                        reverse,
                    )
                })
            } else {
                dst.sum_n(&mut (0..domain.card), |inner, value| {
                    let mut current_values = values.clone();
                    current_values.insert(id, value);

                    (
                        fo2c_recusive(
                            src.inner().unwrap(),
                            inner,
                            grounded,
                            domains,
                            &current_values,
                            reverse,
                        ),
                        1.0,
                    )
                })
            }
        }
    }
}

pub fn first_order_to_circuit(
    tree: &FirstOrderTree,
    domains: &[Integer],
) -> ProbabilisticCircuitTree {
    tree.compile(|src, dst| {
        let grounded = Grounded::ground(tree, dst.array, domains).unwrap();
        fo2c_recusive(src, dst, &grounded, domains, &Default::default(), false)
    })
}
