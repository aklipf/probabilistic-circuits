use crate::{
    logic::propositional::PropositionalTree,
    tree::{Addr, IndexedMutRef, IndexedRef},
};

use super::{PLogic, PMut, PRef};

fn p2nnf_recusive(
    src: IndexedRef<PropositionalTree>,
    dst: &mut IndexedMutRef<PropositionalTree>,
    reverse: bool,
) -> Addr {
    match src.as_ref().value {
        PLogic::Variable { id } => {
            if reverse {
                dst.not(|inner| inner.var(id))
            } else {
                dst.var(id)
            }
        }
        PLogic::Not => p2nnf_recusive(src.inner(), dst, !reverse),
        PLogic::And => {
            if reverse {
                dst.or(
                    |left| p2nnf_recusive(src.left(), left, reverse),
                    |right| p2nnf_recusive(src.right(), right, reverse),
                )
            } else {
                dst.and(
                    |left| p2nnf_recusive(src.left(), left, reverse),
                    |right| p2nnf_recusive(src.right(), right, reverse),
                )
            }
        }
        PLogic::Or => {
            if reverse {
                dst.and(
                    |left| p2nnf_recusive(src.left(), left, reverse),
                    |right| p2nnf_recusive(src.right(), right, reverse),
                )
            } else {
                dst.or(
                    |left| p2nnf_recusive(src.left(), left, reverse),
                    |right| p2nnf_recusive(src.right(), right, reverse),
                )
            }
        }
    }
}

pub fn propositional_to_nnf(tree: &PropositionalTree) -> PropositionalTree {
    tree.compile(|src, dst| {
        dst.array.copy_named(src.array);
        p2nnf_recusive(src, dst, false)
    })
}
