use crate::{
    logic::{
        first_order::{FOLogic, FORef, FirstOrderTree},
        propositional::{PLogic, PRef, PropositionalTree},
    },
    solver::domain::Integer,
    tree::{Addr, IndexedMutRef, IndexedRef, Mapping},
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
    reverse: bool,
) -> Addr {
    match src.as_ref().value {
        FOLogic::Not => fo2c_recusive(src.inner(), dst, !reverse),
        FOLogic::And => {
            if reverse {
                dst.sum(
                    |left| fo2c_recusive(src.left(), left, reverse),
                    |right| fo2c_recusive(src.right(), right, reverse),
                )
            } else {
                dst.prod(
                    |left| fo2c_recusive(src.left(), left, reverse),
                    |right| fo2c_recusive(src.right(), right, reverse),
                )
            }
        }
        FOLogic::Or => {
            if reverse {
                dst.prod(
                    |left| fo2c_recusive(src.left(), left, reverse),
                    |right| fo2c_recusive(src.right(), right, reverse),
                )
            } else {
                dst.sum(
                    |left| fo2c_recusive(src.left(), left, reverse),
                    |right| fo2c_recusive(src.right(), right, reverse),
                )
            }
        }
        FOLogic::Predicate { id } => todo!(),
        FOLogic::Universal { id } => todo!(),
        FOLogic::Existential { id } => todo!(),
    }
}

pub fn first_order_to_circuit(
    tree: &FirstOrderTree,
    domains: &[Integer],
) -> ProbabilisticCircuitTree {
    tree.compile(|src, dst| fo2c_recusive(src, dst, false))
}
