use nnf::propositional_to_nnf;

use crate::{logic::semantic::Eval, tree::IntoAddr};

use super::*;

#[test]
fn build() {
    assert_eq!(
        format!("{}", PropositionalTree::build(|builder| builder.var("A"))),
        "A"
    );
    assert_eq!(
        format!(
            "{}",
            PropositionalTree::build(|builder| builder.not(|inner| inner.var("A")))
        ),
        "\u{00AC}A"
    );
    assert_eq!(
        format!(
            "{}",
            PropositionalTree::build(|builder| builder.and(
                |left| left.not(|inner| inner.var("A")),
                |right| right.var("B")
            ))
        ),
        "(\u{00AC}A\u{2227}B)"
    );
    assert_eq!(
        format!(
            "{}",
            PropositionalTree::build(|builder| builder.or(
                |left| left.not(|inner| inner.not(|inner| inner.var("A"))),
                |right| right.var("B")
            ))
        ),
        "(\u{00AC}\u{00AC}A\u{2228}B)"
    );
    assert_eq!(
        format!(
            "{}",
            PropositionalTree::build(|builder| builder.or(
                |left| left.var("A"),
                |right| right.and(
                    |left| left.var("B"),
                    |right| right.not(|inner| inner.var("B"))
                )
            ))
        ),
        "(A\u{2228}(B\u{2227}\u{00AC}B))"
    );
}

#[test]
fn eval() {
    let mut tree = PropositionalTree::build(|builder| {
        builder.or(
            |left| left.var("A"),
            |right| {
                right.and(
                    |left| left.var("B"),
                    |right| right.not(|inner| inner.var("C")),
                )
            },
        )
    });

    for x in 0..8 {
        let assignment = vec![x & 1 != 0, x & 2 != 0, x & 4 != 0];
        let a = assignment["A".get_addr(&mut tree).addr()];
        let b = assignment["B".get_addr(&mut tree).addr()];
        let c = assignment["C".get_addr(&mut tree).addr()];

        assert_eq!(tree.eval(&assignment), a || (b && (!c)));
    }
}

#[test]
fn nnf() {
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

    let nnf=propositional_to_nnf(&input);
    assert_eq!(format!("{nnf}"),"((A∧(A∨((B∨¬C)∨(A∧C))))∧(D∧¬B))");
}
