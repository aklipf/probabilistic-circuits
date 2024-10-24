use crate::{
    logic::propositional::{PLogic, PRef, PropositionalTree},
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
    tree.compile(|src, dst| p2c_recusive(src, dst, false), true)
}
