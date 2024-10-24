use crate::{logic::semantic::Eval, tree::IndexedRef};

use super::{PCRef, PCicruit, ProbabilisticCircuitTree};

impl<'a> Eval<bool> for IndexedRef<'a, ProbabilisticCircuitTree> {
    type Output = f32;

    fn eval(&self, assignment: &Vec<bool>) -> Self::Output {
        match self.as_ref().value {
            PCicruit::Variable { id, .. } => {
                if assignment[id.addr()] {
                    1.0
                } else {
                    0.0
                }
            }
            PCicruit::Product => self.left().eval(assignment) * self.right().eval(assignment),
            PCicruit::Sum { left, right } => {
                (left * self.left().eval(assignment)) + (right * self.right().eval(assignment))
            }
        }
    }
}

impl Eval<bool> for ProbabilisticCircuitTree {
    type Output = f32;

    fn eval(&self, assignment: &Vec<bool>) -> Self::Output {
        self.output().eval(assignment)
    }
}
