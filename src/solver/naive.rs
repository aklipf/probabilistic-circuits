use std::marker::PhantomData;

use crate::{
    logic::propositional::{eval::Eval, PropositionalLogic},
    tree::{index::Indexing, traits::Mapping, tree::ExpressionTree},
};

pub struct Enumerate<'a, I: Indexing, T: ExpressionTree<PropositionalLogic<I>, I, 2> + Eval<bool>> {
    expr: &'a T,
    num_variables: usize,
    current_solution: usize,
    _marker: PhantomData<I>,
}

impl<'a, I: Indexing, T: ExpressionTree<PropositionalLogic<I>, I, 2> + Eval<bool>>
    Enumerate<'a, I, T>
{
    pub fn domain_size(&self) -> usize {
        1 << self.num_variables
    }
}

impl<'a, I: Indexing, T: ExpressionTree<PropositionalLogic<I>, I, 2> + Eval<bool>> Iterator
    for Enumerate<'a, I, T>
{
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

pub fn enumerate<
    'a,
    I: Indexing,
    T: ExpressionTree<PropositionalLogic<I>, I, 2> + Mapping<I> + Eval<bool>,
>(
    expr: &'a T,
) -> Enumerate<'a, I, T> {
    assert!(expr.num_named() <= (usize::BITS - 1) as usize);

    Enumerate {
        expr: expr,
        num_variables: expr.num_named(),
        current_solution: 0,
        _marker: Default::default(),
    }
}
