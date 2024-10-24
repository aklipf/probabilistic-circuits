use crate::{logic::propositional::Eval, tree::Mapping};

pub struct Enumerate<'a, T: Eval<bool>> {
    expr: &'a T,
    num_variables: usize,
    current_solution: usize,
}

impl<'a, T: Eval<bool>> Enumerate<'a, T> {
    pub fn domain_size(&self) -> usize {
        1 << self.num_variables
    }
}

impl<'a, T: Eval<bool>> Iterator for Enumerate<'a, T> {
    type Item = Vec<bool>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current_solution < self.domain_size() {
            let assignment: Vec<bool> = (0..self.num_variables)
                .map(|x| ((1 << x) & self.current_solution) != 0)
                .collect();

            self.current_solution += 1;

            if self.expr.eval(&assignment) {
                return Some(assignment);
            }
        }
        None
    }
}

pub fn enumerate<'a, T: Mapping + Eval<bool>>(expr: &'a T) -> Enumerate<'a, T> {
    assert!(expr.num_named() <= (usize::BITS - 1) as usize);

    Enumerate {
        expr: expr,
        num_variables: expr.num_named(),
        current_solution: 0,
    }
}
