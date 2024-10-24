use crate::logic::propositional::{PMut, PropositionalTree};
use crate::logic::circuit::propositional_to_circuit;




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
