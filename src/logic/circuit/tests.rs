use crate::logic::propositional::{PMut, PropositionalTree};
use crate::logic::circuit::propositional_to_circuit;
use crate::logic::Eval;

use super::{PCMut, ProbabilisticCircuitTree};


#[test]
fn eval(){
    let pc = ProbabilisticCircuitTree::build(|builder| builder.sum(|left| left.prod(|left| left.var("A"), |right| right.not_var("B")), |right| right.sum(|left| left.not_var("A"), |right| right.var("C"))));

    assert_eq!(pc.eval(&vec![false,false,false]),0.0);
    assert_eq!(pc.eval(&vec![true,false,false]),1.0);
    assert_eq!(pc.eval(&vec![false,true,false]),0.0);
    assert_eq!(pc.eval(&vec![true,true,false]),2.0);
    assert_eq!(pc.eval(&vec![false,false,true]),1.0);
    assert_eq!(pc.eval(&vec![true,false,true]),2.0);
    assert_eq!(pc.eval(&vec![false,true,true]),1.0);
    assert_eq!(pc.eval(&vec![true,true,true]),3.0);
}

#[test]
fn compilation() {
    let input = PropositionalTree::build(|builder| {
        builder.not(|inner| {
            inner.not(|inner| {
                inner.and(
                    |left| {
                        left.not(|inner| {
                            inner.or(
                                |left| left.not(|inner| inner.var("A")),
                                |right| {
                                    right.and(
                                        |left| left.not(|inner| inner.var("A")),
                                        |right| {
                                            right.and(
                                                |left| {
                                                    left.and(
                                                        |left| left.not(|inner| inner.var("B")),
                                                        |right| right.var("C"),
                                                    )
                                                },
                                                |right| {
                                                    right.not(|inner| {
                                                        inner.and(
                                                            |left| left.var("A"),
                                                            |right| right.var("C"),
                                                        )
                                                    })
                                                },
                                            )
                                        },
                                    )
                                },
                            )
                        })
                    },
                    |right| {
                        right.not(|inner| {
                            inner.or(
                                |left| left.not(|inner| inner.var("D")),
                                |right| right.var("B"),
                            )
                        })
                    },
                )
            })
        })
    });

    let pc=propositional_to_circuit(&input);
    assert_eq!(format!("{pc}"),"((A*(A+((B+¬C)+(A*C))))*(D*¬B))");
}
