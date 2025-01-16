use crate::{logic::semantic::Eval, tree::IndexedRef};

use super::{PLogic, PRef, PropositionalTree};

impl<'a> Eval<bool> for IndexedRef<'a, PropositionalTree> {
    type Output = bool;

    fn eval(&self, assignment: &Vec<bool>) -> Self::Output {
        match self.as_ref().value {
            PLogic::Variable { id } => assignment[id.addr()],
            PLogic::Not => !self.inner().eval(assignment),
            PLogic::And => self.left().eval(assignment) && self.right().eval(assignment),
            PLogic::Or => self.left().eval(assignment) || self.right().eval(assignment),
        }
    }
}

impl Eval<bool> for PropositionalTree {
    type Output = bool;

    fn eval(&self, assignment: &Vec<bool>) -> Self::Output {
        self.output().eval(assignment)
    }
}
